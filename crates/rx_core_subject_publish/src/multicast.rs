use std::{
	collections::HashMap,
	sync::{Arc, Mutex, MutexGuard},
};

use derive_where::derive_where;
use rx_core_traits::{
	LockWithPoisonBehavior, Observer, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike,
};

use crate::internal::{
	MulticastAddLockError, MulticastCompleteLockError, MulticastErrorLockError,
	MulticastNextLockError, MulticastNotification, MulticastSubscriberId,
	MulticastSubscriberIdGenerator, MulticastUnsubscribeLockError,
};

pub(crate) const MULTICAST_MAX_RECURSION_DEPTH: usize = 10;

#[derive_where(Default, Debug)]
pub(crate) struct Subscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip)]
	pub(crate) subscribers:
		HashMap<MulticastSubscriberId, Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>,
}

// TODO: This could be real subscriber impls
impl<In, InError> Subscribers<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	// TODO: Reintroduce this in a safe place where individual subscribers are safe to lock, it should be okay just at the end of next/error etc fns
	#[inline]
	pub(crate) fn clean(&mut self) {
		self.subscribers
			.retain(|_, subscriber| !subscriber.is_closed());
	}

	pub(crate) fn add_subscriber(
		&mut self,
		id: MulticastSubscriberId,
		subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>,
	) {
		self.subscribers.insert(id, subscriber);
	}

	pub(crate) fn apply(&mut self, notification: MulticastNotification<In, InError>) {
		match notification {
			MulticastNotification::Next(next) => self.next(next),
			MulticastNotification::Error(error) => self.error(error),
			MulticastNotification::Complete => self.complete(),
			MulticastNotification::Unsubscribe => self.unsubscribe(),
			MulticastNotification::UnsubscribeById(id) => self.unsubscribe_by_id(id),
			MulticastNotification::Add(id, subscriber) => {
				self.add_subscriber(id, subscriber);
			}
		}
	}

	pub(crate) fn next(&mut self, next: In) {
		for (_, destination) in self.subscribers.iter_mut() {
			if !destination.is_closed() {
				destination.next(next.clone());
			}
		}
	}

	pub(crate) fn error(&mut self, error: InError) {
		for (_, destination) in self.subscribers.iter_mut() {
			if !destination.is_closed() {
				destination.error(error.clone());
				destination.unsubscribe();
			}
		}
	}

	pub(crate) fn complete(&mut self) {
		for (_, destination) in self.subscribers.iter_mut() {
			if !destination.is_closed() {
				destination.complete();
				destination.unsubscribe();
			}
		}
	}

	pub(crate) fn unsubscribe(&mut self) {
		for (_, mut destination) in self.subscribers.drain() {
			if !destination.is_closed() {
				destination.unsubscribe();
			}
		}
	}

	pub(crate) fn unsubscribe_by_id(&mut self, id: MulticastSubscriberId) {
		if let Some(mut destination) = self.subscribers.remove(&id)
			&& !destination.is_closed()
		{
			destination.unsubscribe();
		}
	}
}

#[derive_where(Clone)]
#[derive(Debug)]
pub(crate) struct SharedSubscribers<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub(crate) deferred_state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
	pub(crate) subscriber_id_generator: Arc<Mutex<MulticastSubscriberIdGenerator>>,
	pub(crate) subscribers: Arc<Mutex<Subscribers<In, InError>>>,
}

impl<In, InError> SharedSubscribers<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub(crate) fn new(
		deferred_state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
		subscriber_id_generator: Arc<Mutex<MulticastSubscriberIdGenerator>>,
	) -> Self {
		Self {
			deferred_state,
			subscriber_id_generator,
			subscribers: Arc::new(Mutex::new(Subscribers::default())),
		}
	}

	// TODO: Too similar fn to other apply_notification_queue's
	pub(crate) fn apply_notification_queue(
		state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
		subscribers: &mut MutexGuard<'_, Subscribers<In, InError>>,
	) {
		for queue_depth in 0..=MULTICAST_MAX_RECURSION_DEPTH {
			let notifications = {
				let mut locked_state = state.lock_ignore_poison();

				// Infinite loop protection
				if queue_depth == MULTICAST_MAX_RECURSION_DEPTH {
					panic!(
						"Notification queue depth have exceeded {MULTICAST_MAX_RECURSION_DEPTH}!"
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
			for mut notification in notifications.into_iter() {
				if let MulticastNotification::Add(_id, subscriber) = &mut notification {
					if subscriber.is_closed() {
						// Don't add an already closed subscriber, just drop it
						continue;
					} else if state.lock_ignore_poison().is_closed_ignoring_deferred() {
						// If the subscriber isn't closed, but the state is
						// the subscriber should be unsubscribed too
						subscriber.unsubscribe();
						continue;
					}
				}

				let is_complete = matches!(&notification, MulticastNotification::Complete);

				let is_error = matches!(&notification, MulticastNotification::Error(_));

				let is_terminal = is_complete || is_error;

				let is_unsubscribe = matches!(&notification, MulticastNotification::Unsubscribe);

				// Other notifications can be safely dropped when already closed
				if !state.lock_clear_poison().is_closed_ignoring_deferred() {
					subscribers.apply(notification);
				}

				if is_terminal {
					subscribers.apply(MulticastNotification::Unsubscribe);
				}

				if is_unsubscribe || is_terminal {
					state.lock_ignore_poison().closed_flag.close();
				}
			}

			subscribers.clean();
		}
	}

	pub(crate) fn try_add_subscriber(
		&mut self,
		subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>,
	) -> Result<MulticastSubscriberId, MulticastAddLockError<In, InError>> {
		let id = self.subscriber_id_generator.lock_ignore_poison().get_next();

		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.add_subscriber(id, subscriber);

				Ok(id)
			}
			Err(_) => Err(MulticastAddLockError { id, subscriber }),
		}
	}

	pub(crate) fn try_next(&mut self, next: In) -> Result<(), MulticastNextLockError<In>> {
		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.next(next);

				Ok(())
			}
			Err(_) => Err(MulticastNextLockError { next }),
		}
	}

	pub(crate) fn try_error(
		&mut self,
		error: InError,
	) -> Result<(), MulticastErrorLockError<InError>> {
		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.error(error);

				Ok(())
			}
			Err(_) => Err(MulticastErrorLockError { error }),
		}
	}

	pub(crate) fn try_complete(&mut self) -> Result<(), MulticastCompleteLockError> {
		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.complete();
				Ok(())
			}
			Err(_) => Err(MulticastCompleteLockError),
		}
	}

	pub(crate) fn try_unsubscribe(&mut self) -> Result<(), MulticastUnsubscribeLockError> {
		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				self.deferred_state.lock_ignore_poison().closed_flag.close();

				subscribers.unsubscribe();

				Ok(())
			}
			Err(_) => Err(MulticastUnsubscribeLockError),
		}
	}

	pub(crate) fn try_unsubscribe_by_id(
		&mut self,
		id: MulticastSubscriberId,
	) -> Result<(), MulticastUnsubscribeLockError> {
		match self.subscribers.try_lock() {
			Ok(mut subscribers) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscribers); // First, the notification queue!

				subscribers.unsubscribe_by_id(id);

				Ok(())
			}
			Err(_) => Err(MulticastUnsubscribeLockError),
		}
	}
}

#[derive(Debug)]
pub(crate) struct MulticastDeferredState<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub(crate) deferred_notifications_queue: Vec<MulticastNotification<In, InError>>,

	/// Separate close flag for the real, applied closedness, as non-deferred
	/// signals only have to respect this.
	pub(crate) closed_flag: SubscriptionClosedFlag,
	/// This flag is only meant to block incoming notifications, if an unsubscribe
	/// had already observed, to not accept more.
	pub(crate) observed_unsubscribe: bool,
	/// Signals if Completion has been observed or not
	pub(crate) observed_completion: bool,
	/// Signals if an error was observed or not
	pub(crate) observed_error: Option<InError>,
}

impl<In, InError> MulticastDeferredState<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// TODO: Needs tests to see how self feeding subscriptions react with this
	pub(crate) fn defer_notification(&mut self, notification: MulticastNotification<In, InError>) {
		// The first unsubscribe notification must be let through
		let is_first_unsubscribe = matches!(notification, MulticastNotification::Unsubscribe)
			&& !self.observed_unsubscribe;

		// TODO: SUBJECTG DONT UNSUB UNLESS MANUALLK
		if *self.closed_flag && !is_first_unsubscribe {
			if let MulticastNotification::Add(_id, mut subscriber) = notification
				&& !subscriber.is_closed()
			{
				subscriber.unsubscribe();
			}

			return;
		}

		self.deferred_notifications_queue.push(notification);
	}

	pub(crate) fn drain_notification_queue(&mut self) -> Vec<MulticastNotification<In, InError>> {
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
			|| self.observed_error.is_some()
	}

	/// Is actually closed, ignoring currently deferred notifications
	pub(crate) fn is_closed_ignoring_deferred(&self) -> bool {
		*self.closed_flag
	}
}

impl<In, InError> Drop for MulticastDeferredState<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// The flag might not be closed on drop
		self.closed_flag.close();

		debug_assert!(!self.is_dirty(), "MulticastState was dropped dirty!");
	}
}

impl<In, InError> Default for MulticastDeferredState<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			closed_flag: false.into(),
			observed_completion: false,
			observed_unsubscribe: false,
			observed_error: None,
			deferred_notifications_queue: Vec::default(),
		}
	}
}

#[cfg(test)]
mod test {
	use std::sync::{Arc, Mutex};

	use rx_core_testing::MockObserver;
	use rx_core_traits::{LockWithPoisonBehavior, Never, SubscriptionLike};

	use crate::internal::{
		MulticastDeferredState, MulticastNotification, MulticastSubscriberIdGenerator,
		SharedSubscribers,
	};

	#[test]
	fn should_unsubscribe_deferred_subscriber_adds_when_adding_to_an_already_closed_multicast() {
		let mut multicast_state = MulticastDeferredState::<usize, Never>::default();
		let shared_destination = Arc::new(Mutex::new(MockObserver::default()));
		let mut subscriber_id_generator = MulticastSubscriberIdGenerator::default();

		multicast_state.closed_flag.close();
		assert!(!shared_destination.is_closed());

		multicast_state.defer_notification(MulticastNotification::Add(
			subscriber_id_generator.get_next(),
			shared_destination.clone(),
		));
		assert!(shared_destination.is_closed());
	}

	#[test]
	fn should_unsubscribe_deferred_subscriber_if_multicast_was_closed_between_defer_and_apply() {
		let multicast_state =
			Arc::new(Mutex::new(MulticastDeferredState::<usize, Never>::default()));

		let subscriber_id_generator =
			Arc::new(Mutex::new(MulticastSubscriberIdGenerator::default()));

		let id = subscriber_id_generator.lock_ignore_poison().get_next();

		let shared_subscribers =
			SharedSubscribers::new(multicast_state.clone(), subscriber_id_generator.clone());
		let shared_destination = Arc::new(Mutex::new(MockObserver::default()));

		assert!(!shared_destination.is_closed());

		multicast_state
			.lock_ignore_poison()
			.defer_notification(MulticastNotification::Add(id, shared_destination.clone()));

		assert!(!shared_destination.is_closed());

		multicast_state.lock_ignore_poison().closed_flag.close();

		SharedSubscribers::apply_notification_queue(
			multicast_state,
			&mut shared_subscribers.subscribers.lock_ignore_poison(),
		);

		assert!(shared_destination.is_closed());
	}

	#[test]
	fn should_unsubscribe_subscribers_when_a_deferred_terminal_complete_signal_is_applied() {
		let multicast_state =
			Arc::new(Mutex::new(MulticastDeferredState::<usize, Never>::default()));

		let subscriber_id_generator =
			Arc::new(Mutex::new(MulticastSubscriberIdGenerator::default()));

		let mut shared_subscribers =
			SharedSubscribers::new(multicast_state.clone(), subscriber_id_generator);
		let shared_destination = Arc::new(Mutex::new(MockObserver::<usize, Never>::default()));
		shared_subscribers
			.try_add_subscriber(shared_destination.clone())
			.expect("to be successful");

		assert!(!shared_destination.is_closed());

		multicast_state
			.lock_ignore_poison()
			.defer_notification(MulticastNotification::Complete);

		assert!(!shared_destination.is_closed());

		SharedSubscribers::apply_notification_queue(
			multicast_state,
			&mut shared_subscribers.subscribers.lock_ignore_poison(),
		);
		assert!(shared_destination.is_closed());
	}

	#[test]
	fn should_unsubscribe_subscribers_when_a_deferred_terminal_error_signal_is_applied() {
		let multicast_state = Arc::new(Mutex::new(
			MulticastDeferredState::<usize, &'static str>::default(),
		));

		let subscriber_id_generator =
			Arc::new(Mutex::new(MulticastSubscriberIdGenerator::default()));

		let mut shared_subscribers =
			SharedSubscribers::new(multicast_state.clone(), subscriber_id_generator);

		let shared_destination =
			Arc::new(Mutex::new(MockObserver::<usize, &'static str>::default()));
		shared_subscribers
			.try_add_subscriber(shared_destination.clone())
			.expect("to be successful");

		assert!(!shared_destination.is_closed());

		multicast_state
			.lock_ignore_poison()
			.defer_notification(MulticastNotification::Error("error"));

		assert!(!shared_destination.is_closed());

		SharedSubscribers::apply_notification_queue(
			multicast_state,
			&mut shared_subscribers.subscribers.lock_ignore_poison(),
		);
		assert!(shared_destination.is_closed());
	}
}
