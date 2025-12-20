use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	LockWithPoisonBehavior, Never, Observable, Observer, Signal, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};

use crate::internal::{
	MulticastNotification, MulticastState, MulticastSubscription, SharedSubscribers,
};

/// # [PublishSubject]
///
/// Multicasting primitive.
///
/// A PublishSubject is a shared multicast observer, can be used for
/// broadcasting events to multiple concurrent subscribers.
///
/// > A subjects clone still multicasts to the same set of subscribers.
///
/// ## Circular Signals
///
/// Sometimes a subjects subscription would like to interact with the subject
/// itself, like unsubscribing it from a teardown.
/// And while not recommended, it should be able to feed values too back into
/// itself. Which while is infinite-loop prone, still better than just
/// deadlocking the entire thread.
///
/// This is achieved by deferring notifications into a queue when the
/// subscribers are locked. The (internal) holder of the lock will always
/// check the queue too for unprocessed notifications.
#[derive_where(Clone)]
#[derive(RxSubject, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct PublishSubject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	subscribers: SharedSubscribers<In, InError>,
	state: Arc<Mutex<MulticastState<In, InError>>>,
}

impl<In, InError> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Drops all closed subscribers
	fn try_clean(&mut self) {
		if self.state.lock_ignore_poison().is_dirty()
			&& let Ok(mut subscribers) = self.subscribers.subscribers.try_lock()
		{
			SharedSubscribers::apply_notification_queue(self.state.clone(), &mut subscribers);
		}
	}
}

impl<In, InError> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	pub fn is_errored(&self) -> bool {
		let state = self.state.lock_ignore_poison();
		state.observed_error.is_some()
	}
}

impl<In, InError> Observable for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		// In case a deferred notification would close this subject
		self.try_clean();

		let mut subscriber = destination.upgrade();

		{
			let state = self.state.lock_ignore_poison();
			if let Some(error) = state.observed_error.clone() {
				subscriber.error(error);
			} else if state.observed_completion {
				subscriber.complete();
			}
		}

		if self.is_closed() {
			subscriber.unsubscribe();
			MulticastSubscription::new_closed()
		} else {
			let shared_subscriber = Arc::new(Mutex::new(subscriber));
			let subscriber_clone = shared_subscriber.clone();
			if let Err(add_subscriber_error) =
				self.subscribers.try_add_subscriber(shared_subscriber)
			{
				self.state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Add(
						add_subscriber_error.subscriber,
					));
			} else {
				// In case the new subscription immediately did something with
				// this subject
				self.try_clean();
			}

			MulticastSubscription::new(subscriber_clone)
		}
	}
}

impl<In, InError> Observer for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		self.try_clean();

		if !self.is_closed()
			&& let Err(next_error) = self.subscribers.try_next(next)
		{
			self.state
				.lock_ignore_poison()
				.defer_notification(MulticastNotification::Next(next_error.next));
		}

		self.try_clean();
	}

	fn error(&mut self, error: Self::InError) {
		self.try_clean();

		if !self.is_closed() {
			self.state.lock_ignore_poison().observed_error = Some(error.clone());

			if let Err(error_error) = self.subscribers.try_error(error) {
				self.state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Error(error_error.error));
			}

			self.unsubscribe();
		}

		self.try_clean();
	}

	fn complete(&mut self) {
		self.try_clean();

		if !self.is_closed() {
			self.state.lock_ignore_poison().observed_completion = true;

			if let Err(_complete_error) = self.subscribers.try_complete() {
				self.state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Complete);
			}

			self.unsubscribe();
		}

		self.try_clean();
	}
}

impl<In, InError> SubscriptionLike for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.state.lock_ignore_poison().is_closed()
	}

	fn unsubscribe(&mut self) {
		self.try_clean();

		// TODO: Optimize locks once tested and working
		if !self.is_closed() {
			self.state.lock_ignore_poison().observed_unsubscribe = true;

			println!("UNSUB??");

			if let Err(_unsubscribe_error) = self.subscribers.try_unsubscribe() {
				println!("DEFERRED UNSUBV??");

				self.state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Unsubscribe);
			}
		}

		self.try_clean();
	}
}

impl<In, InError> Default for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		let shared_multicast_state = Arc::new(Mutex::new(MulticastState::default()));
		Self {
			subscribers: SharedSubscribers::new(shared_multicast_state.clone()),
			state: shared_multicast_state,
		}
	}
}

impl<In, InError> Drop for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// Does not need to unsubscribe on drop as it's shared
		// But it does need to check for unprocessed notifications.
		self.try_clean();
	}
}
