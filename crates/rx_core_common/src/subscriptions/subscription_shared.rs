use std::sync::{Arc, Mutex, MutexGuard};

use rx_core_macro_subscription_derive::RxSubscription;

use crate::{
	LockWithPoisonBehavior, SubscriptionClosedFlag, SubscriptionData, SubscriptionLike,
	SubscriptionLikePushNotificationExtention, SubscriptionNotification,
	TeardownCollectionExtension,
};

pub(crate) const SUBSCRIPTION_MAX_RECURSION_DEPTH: usize = 10;

#[derive(Debug)]
pub(crate) struct SubscriptionUnsubscribeLockError;

#[derive(Debug)]
struct SubscriptionDeferredState {
	pub(crate) deferred_notifications_queue: Vec<SubscriptionNotification>,

	/// Separate close flag for the real, applied closedness, as non-deferred
	/// signals only have to respect this.
	pub(crate) closed_flag: SubscriptionClosedFlag,
	/// This flag is only meant to block incoming notifications, if an unsubscribe
	/// had already observed, to not accept more.
	pub(crate) observed_unsubscribe: bool,
}

impl SubscriptionDeferredState {
	pub(crate) fn defer_notification(&mut self, notification: SubscriptionNotification) {
		// The first unsubscribe notification must be let through
		let is_first_unsubscribe = matches!(notification, SubscriptionNotification::Unsubscribe)
			&& !self.observed_unsubscribe;

		if *self.closed_flag && !is_first_unsubscribe {
			return;
		}

		self.deferred_notifications_queue.push(notification);
	}

	pub(crate) fn drain_notification_queue(&mut self) -> Vec<SubscriptionNotification> {
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
		self.is_closed_ignoring_deferred() || self.observed_unsubscribe
	}

	/// Is actually closed, ignoring currently deferred notifications
	pub(crate) fn is_closed_ignoring_deferred(&self) -> bool {
		*self.closed_flag
	}
}

impl Drop for SubscriptionDeferredState {
	fn drop(&mut self) {
		// The flag might not be closed on drop
		self.closed_flag.close();

		debug_assert!(
			!self.is_dirty(),
			"SubscriptionDeferredState was dropped dirty!"
		);
	}
}

impl Default for SubscriptionDeferredState {
	fn default() -> Self {
		Self {
			closed_flag: false.into(),
			observed_unsubscribe: false,
			deferred_notifications_queue: Vec::default(),
		}
	}
}

#[derive(RxSubscription, Default, Clone, Debug)]
#[_rx_core_common_crate(crate)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl] // It's shared
pub struct SharedSubscription {
	#[destination]
	subscription: Arc<Mutex<SubscriptionData>>,
	deferred_state: Arc<Mutex<SubscriptionDeferredState>>,
}

impl SharedSubscription {
	pub fn new<S>(subscription: S) -> Self
	where
		S: 'static + SubscriptionLike + Send + Sync,
	{
		let mut default = Self::default();
		default.add(subscription);
		default
	}

	fn try_apply_deferred(&mut self) {
		if self.deferred_state.lock_ignore_poison().is_dirty()
			&& let Ok(mut subscription) = self.subscription.try_lock()
		{
			SharedSubscription::apply_notification_queue(
				self.deferred_state.clone(),
				&mut subscription,
			);
		}
	}

	fn apply_notification_queue(
		state: Arc<Mutex<SubscriptionDeferredState>>,
		subscriber: &mut MutexGuard<'_, SubscriptionData>,
	) {
		for queue_depth in 0..=SUBSCRIPTION_MAX_RECURSION_DEPTH {
			let notifications = {
				let mut locked_state = state.lock_ignore_poison();

				// Infinite loop protection
				if queue_depth == SUBSCRIPTION_MAX_RECURSION_DEPTH {
					panic!(
						"Notification queue depth have exceeded {SUBSCRIPTION_MAX_RECURSION_DEPTH}!"
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
				let is_unsubscribe = matches!(&notification, SubscriptionNotification::Unsubscribe);

				// Other notifications can be safely dropped when already closed
				if !state.lock_clear_poison().is_closed_ignoring_deferred() {
					subscriber.push(notification);
				}

				if is_unsubscribe {
					state.lock_ignore_poison().closed_flag.close();
				}
			}
		}
	}

	pub(crate) fn try_unsubscribe(&mut self) -> Result<(), SubscriptionUnsubscribeLockError> {
		match self.subscription.try_lock() {
			Ok(mut subscription) => {
				Self::apply_notification_queue(self.deferred_state.clone(), &mut subscription); // First, the notification queue!

				self.deferred_state.lock_ignore_poison().closed_flag.close();

				if !subscription.is_closed() {
					subscription.unsubscribe();
				}

				Ok(())
			}
			Err(_) => Err(SubscriptionUnsubscribeLockError),
		}
	}
}

impl SubscriptionLike for SharedSubscription {
	fn is_closed(&self) -> bool {
		self.deferred_state.lock_ignore_poison().is_closed()
			|| self
				.subscription
				.try_lock()
				.map(|s| s.is_closed())
				.unwrap_or(false)
	}

	fn unsubscribe(&mut self) {
		self.try_apply_deferred();

		let was_unsubscribed = {
			let mut state = self.deferred_state.lock_ignore_poison();
			let was_unsubscribed = state.is_unsubscribed();
			state.observed_unsubscribe = true;
			was_unsubscribed
		};

		if !was_unsubscribed && let Err(_unsubscribe_error) = self.try_unsubscribe() {
			self.deferred_state
				.lock_ignore_poison()
				.defer_notification(SubscriptionNotification::Unsubscribe);
		}

		self.try_apply_deferred();
	}
}

impl Drop for SharedSubscription {
	fn drop(&mut self) {
		// Don't do anything, it's shared
		self.try_apply_deferred();
	}
}
