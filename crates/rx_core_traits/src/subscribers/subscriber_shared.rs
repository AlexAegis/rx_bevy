use std::sync::{Arc, Mutex, MutexGuard, Weak};

use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{
	LockWithPoisonBehavior, Observer, SharedDestination, Signal, Subscriber,
	SubscriberNotification, SubscriberPushNotificationExtention, SubscriptionClosedFlag,
	SubscriptionLike, UpgradeableObserver,
};

pub(crate) const SUBSCRIBER_MAX_RECURSION_DEPTH: usize = 10;

#[derive(Debug)]
pub(crate) struct SubscriberNextLockError<In>
where
	In: Signal,
{
	pub(crate) next: In,
}

#[derive(Debug)]
pub(crate) struct SubscriberErrorLockError<InError>
where
	InError: Signal,
{
	pub(crate) error: InError,
}

#[derive(Debug)]
pub(crate) struct SubscriberCompleteLockError;

#[derive(Debug)]
pub(crate) struct SubscriberUnsubscribeLockError;

#[derive(Debug)]
struct SubscriberDeferredState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub(crate) deferred_notifications_queue: Vec<SubscriberNotification<In, InError>>,

	/// Separate close flag for the real, applied closedness, as non-deferred
	/// signals only have to respect this.
	pub(crate) closed_flag: SubscriptionClosedFlag,
	/// This flag is only meant to block incoming notifications, if an unsubscribe
	/// had already observed, to not accept more.
	pub(crate) observed_unsubscribe: bool,
	/// Signals if Completion has been observed or not
	pub(crate) observed_completion: bool,
	/// Signals if an error was observed or not
	pub(crate) observed_error: bool,
}

impl<In, InError> SubscriberDeferredState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub(crate) fn defer_notification(&mut self, notification: SubscriberNotification<In, InError>) {
		// The first unsubscribe notification must be let through
		let is_first_unsubscribe = matches!(notification, SubscriberNotification::Unsubscribe)
			&& !self.observed_unsubscribe;

		if *self.closed_flag && !is_first_unsubscribe {
			return;
		}

		self.deferred_notifications_queue.push(notification);
	}

	pub(crate) fn drain_notification_queue(&mut self) -> Vec<SubscriberNotification<In, InError>> {
		self.deferred_notifications_queue
			.drain(..)
			.collect::<Vec<_>>()
	}

	/// The state is considered dirty when there are unprocessed notifications
	/// in the queue.
	pub(crate) fn is_dirty(&self) -> bool {
		!self.deferred_notifications_queue.is_empty()
	}

	pub(crate) fn is_unsubscribed(&self) -> bool {
		self.observed_unsubscribe
	}

	pub(crate) fn is_closed(&self) -> bool {
		self.is_closed_ignoring_deferred()
			|| self.observed_completion
			|| self.observed_unsubscribe
			|| self.observed_error
	}

	/// Is actually closed, ignoring currently deferred notifications
	pub(crate) fn is_closed_ignoring_deferred(&self) -> bool {
		*self.closed_flag
	}
}

impl<In, InError> Drop for SubscriberDeferredState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn drop(&mut self) {
		// The flag might not be closed on drop
		self.closed_flag.close();

		debug_assert!(!self.is_dirty(), "MulticastState was dropped dirty!");
	}
}

impl<In, InError> Default for SubscriberDeferredState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn default() -> Self {
		Self {
			closed_flag: false.into(),
			observed_completion: false,
			observed_unsubscribe: false,
			observed_error: false,
			deferred_notifications_queue: Vec::default(),
		}
	}
}

/// # [SharedSubscriber]
///
/// This should be the very first thing you reach for when you want to share
/// a subscriber between multiple places. Where each place with a clone of this
/// can be sure to send signals to the exact same destination.
///
/// ## Deadlock Protection
///
/// While Subscriber and other traits are implemented on plain `Arc<Mutex<D>>`s
/// too, and those too have uses of their own, this struct here features
/// deadlock protection in form of a deferred notification queue.
///
/// - If the destination is not locked, the signals just go through.
/// - If the destination is locked, the signal materializes into the queue
///   and when the lock is released by the holder, the queue will clear itself,
///   applying all deferred notifications.
#[derive(Debug, RxSubscriber)]
#[_rx_core_traits_crate(crate)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	#[destination]
	shared_destination: Arc<Mutex<Destination>>,
	deferred_state: Arc<Mutex<SubscriberDeferredState<Destination::In, Destination::InError>>>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			deferred_state: Arc::new(Mutex::new(SubscriberDeferredState::default())),
			shared_destination: Arc::new(Mutex::new(destination)),
		}
	}

	/// Locks the shared destination.
	/// In case it encounters a poison error, the destination is immediately
	/// unsubscribed!
	pub fn lock(&self) -> MutexGuard<'_, Destination> {
		self.shared_destination
			.lock()
			.unwrap_or_else(|poison_error| {
				let mut destination = poison_error.into_inner();
				if !destination.is_closed() {
					destination.unsubscribe();
				}
				destination
			})
	}

	pub fn downgrade(&self) -> Weak<Mutex<Destination>> {
		Arc::downgrade(&self.shared_destination)
	}

	fn try_apply_deferred(&mut self) {
		if self.deferred_state.lock_ignore_poison().is_dirty()
			&& let Ok(mut subscriber) = self.shared_destination.try_lock()
		{
			SharedSubscriber::<Destination>::apply_notification_queue::<Destination>(
				self.deferred_state.clone(),
				&mut subscriber,
			);
		}
	}

	fn apply_notification_queue<D>(
		state: Arc<Mutex<SubscriberDeferredState<D::In, D::InError>>>,
		subscriber: &mut MutexGuard<'_, D>,
	) where
		D: Subscriber,
	{
		for queue_depth in 0..=SUBSCRIBER_MAX_RECURSION_DEPTH {
			let notifications = {
				let mut locked_state = state.lock_ignore_poison();

				// Infinite loop protection
				if queue_depth == SUBSCRIBER_MAX_RECURSION_DEPTH {
					panic!(
						"Notification queue depth have exceeded {SUBSCRIBER_MAX_RECURSION_DEPTH}!"
					)
				}

				if locked_state.deferred_notifications_queue.is_empty() {
					break;
				}

				// Don't drain until the above checks have happened to not drop
				// un-applied notifications.
				// In case that panic above is no longer a panic.
				locked_state.drain_notification_queue()
			};

			// Each closedness check acquires a fresh lock for up-to-date
			// information and immediately releases it to allow applied
			// notifications to acquire it again
			for notification in notifications.into_iter() {
				// TODO: simplify is_closing
				let is_complete = matches!(&notification, SubscriberNotification::Complete);

				let is_error = matches!(&notification, SubscriberNotification::Error(_));

				let is_terminal = is_complete || is_error;

				let is_unsubscribe = matches!(&notification, SubscriberNotification::Unsubscribe);

				// Other notifications can be safely dropped when already closed
				if !state.lock_clear_poison().is_closed_ignoring_deferred() {
					subscriber.push(notification);
				}

				if is_unsubscribe || is_terminal {
					state.lock_ignore_poison().closed_flag.close();
				}
			}
		}
	}

	pub(crate) fn try_next(
		&mut self,
		next: Destination::In,
	) -> Result<(), SubscriberNextLockError<Destination::In>> {
		match self.shared_destination.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.next(next);

				Ok(())
			}
			Err(_) => Err(SubscriberNextLockError { next }),
		}
	}

	pub(crate) fn try_error(
		&mut self,
		error: Destination::InError,
	) -> Result<(), SubscriberErrorLockError<Destination::InError>> {
		match self.shared_destination.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.error(error);

				Ok(())
			}
			Err(_) => Err(SubscriberErrorLockError { error }),
		}
	}

	pub(crate) fn try_complete(&mut self) -> Result<(), SubscriberCompleteLockError> {
		match self.shared_destination.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.complete();
				Ok(())
			}
			Err(_) => Err(SubscriberCompleteLockError),
		}
	}

	pub(crate) fn try_unsubscribe(&mut self) -> Result<(), SubscriberUnsubscribeLockError> {
		match self.shared_destination.try_lock() {
			Ok(mut subscriber) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscriber); // First, the notification queue!

				self.deferred_state.lock_ignore_poison().closed_flag.close();

				if !subscriber.is_closed() {
					subscriber.unsubscribe();
				}

				Ok(())
			}
			Err(_) => Err(SubscriberUnsubscribeLockError),
		}
	}
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			deferred_state: self.deferred_state.clone(),
			shared_destination: self.shared_destination.clone(),
		}
	}
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed()
			&& let Err(next_error) = self.try_next(next)
		{
			self.deferred_state
				.lock_ignore_poison()
				.defer_notification(SubscriberNotification::Next(next_error.next));
		}

		self.try_apply_deferred();
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.deferred_state.lock_ignore_poison().observed_error = true;

			if let Err(error_error) = self.try_error(error) {
				self.deferred_state
					.lock_ignore_poison()
					.defer_notification(SubscriberNotification::Error(error_error.error));
			}
		}
		self.try_apply_deferred();
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.deferred_state.lock_ignore_poison().observed_completion = true;

			if let Err(_complete_error) = self.try_complete() {
				self.deferred_state
					.lock_ignore_poison()
					.defer_notification(SubscriberNotification::Complete);
			}
		}
		self.try_apply_deferred();
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.deferred_state.lock_ignore_poison().is_closed()
			|| self
				.shared_destination
				.try_lock()
				.map(|s| s.is_closed())
				.unwrap_or(false)
	}

	fn unsubscribe(&mut self) {
		let was_unsubscribed = {
			let mut state = self.deferred_state.lock_ignore_poison();
			let was_unsubscribed = state.is_unsubscribed();
			state.observed_unsubscribe = true;
			was_unsubscribed
		};

		if !was_unsubscribed && let Err(_unsubscribe_error) = self.try_unsubscribe() {
			self.deferred_state
				.lock_ignore_poison()
				.defer_notification(SubscriberNotification::Unsubscribe);
		}

		self.try_apply_deferred();
	}
}

impl<Destination> SharedDestination<Destination> for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		self.shared_destination.access(accessor);
	}

	#[inline]
	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		self.shared_destination.access_mut(accessor);
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
		self.try_apply_deferred();
	}
}
