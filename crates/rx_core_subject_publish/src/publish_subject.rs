use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	LockWithPoisonBehavior, Never, Observable, Observer, Provider, Signal, Subscriber,
	SubscriptionLike, UpgradeableObserver,
};

use crate::internal::{
	MulticastDeferredState, MulticastNotification, MulticastSubscriberIdGenerator,
	MulticastSubscription, SharedSubscribers,
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
/// ## Subjects
///
/// Subjects are both observers and observables! Anything they observe is
/// then multicast and sent to each individual subscriber subscribing to
/// this subject.
///
/// ## Example
///
/// ```rs
/// use rx_core::prelude::*;
///
/// let mut subject = PublishSubject::<usize>::default();
/// subject.next(1); // Has no effect, nobody is listening!
/// let mut subscription = subject.subscribe(PrintObserver::new("subject"));
/// subject.next(2); // This will get printed!
/// subscription.unsubscribe();
/// subject.next(3); // This will not get printed anymore!
/// ```
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
	deferred_state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
	subscribers: SharedSubscribers<In, InError>,
}

impl<In, InError> Provider for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Provided = Self;

	fn provide(&self) -> Self::Provided {
		Self::default()
	}
}

impl<In, InError> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn try_clean(&mut self) {
		if self.deferred_state.lock_ignore_poison().is_dirty()
			&& let Ok(mut subscribers) = self.subscribers.subscribers.try_lock()
		{
			SharedSubscribers::apply_notification_queue(
				self.deferred_state.clone(),
				&mut subscribers,
			);
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
		let state = self.deferred_state.lock_ignore_poison();
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
			let state = self.deferred_state.lock_ignore_poison();
			if let Some(error) = state.observed_error.clone() {
				subscriber.error(error);
			} else if state.observed_completion {
				subscriber.complete();
			}
		}

		if self.is_closed() {
			// Subscribing to a closed subject could also cause a panic, but I prefer this
			subscriber.unsubscribe();
			MulticastSubscription::new_closed(self.deferred_state.clone(), self.subscribers.clone())
		} else {
			let shared_subscriber = Arc::new(Mutex::new(subscriber));
			let shared_subscriber_clone = shared_subscriber.clone();

			let try_add_result = self.subscribers.try_add_subscriber(shared_subscriber);
			let id = match try_add_result {
				Ok(id) => id,
				Err(add_subscriber_error) => {
					self.deferred_state.lock_ignore_poison().defer_notification(
						MulticastNotification::Add(
							add_subscriber_error.id,
							add_subscriber_error.subscriber,
						),
					);
					add_subscriber_error.id
				}
			};

			// In case the new subscription immediately did something with
			// this subject
			self.try_clean();

			MulticastSubscription::new(
				id,
				self.deferred_state.clone(),
				self.subscribers.clone(),
				shared_subscriber_clone,
			)
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
			self.deferred_state
				.lock_ignore_poison()
				.defer_notification(MulticastNotification::Next(next_error.next));
		}

		self.try_clean();
	}

	fn error(&mut self, error: Self::InError) {
		self.try_clean();

		if !self.is_closed() {
			self.deferred_state.lock_ignore_poison().observed_error = Some(error.clone());

			if let Err(error_error) = self.subscribers.try_error(error) {
				self.deferred_state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Error(error_error.error));
			}
		}

		self.try_clean();
	}

	fn complete(&mut self) {
		self.try_clean();

		if !self.is_closed() {
			self.deferred_state.lock_ignore_poison().observed_completion = true;

			if let Err(_complete_error) = self.subscribers.try_complete() {
				self.deferred_state
					.lock_ignore_poison()
					.defer_notification(MulticastNotification::Complete);
			}
		}

		self.try_clean();
	}
}

/// A subject is both an observable and an observer, but they are **NOT**
/// subscriptions! And traditionally they do not allow to forcibly unsubscribe
/// all subscribers, besides just marking the subject as closed once completed
/// or errored.
///
/// The reason is, that when a subject is used as a destination of a
/// subscription, it should only forward signals, but should definitely **NOT**
/// unsubscribe the subscribers. That subject could be used as a destination
/// for multiple subscriptions, one should not close it from others, unless it
/// completes or errors it.
///
/// But here that does not happen, because Observables here in `rx_core` expect
/// not an observer, but an `UpgradeableObserver`. And it's up to the
/// destination to decide if it wants to be unsubscribed together with upstream
/// or not. (This exists to be able to use subscribers directly as destinations)
///
/// This trait is autoimplemented by the `RxSubject` macro and makes subjects
/// always be wrapped in a `DetatchedSubscriber` which only forwards signals
/// but not `unsubscribe` calls.
///
/// > Publish, Behavior, Replay and Async subjects all upgrade to a detached
/// > subscriber, but other subjects implement it at their own discretion. The
/// > RxSubject macro intentionally makes this the default behavior, so if a
/// > custom subject does not detach on subscribe, it's most likely intentional.
///
/// So you get to pass subjects as destinations without worrying that it
/// unsubscribes when it shouldn't, but you can still do it by hand and drop
/// all subscribers.
impl<In, InError> SubscriptionLike for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.deferred_state.lock_ignore_poison().is_closed()
	}

	fn unsubscribe(&mut self) {
		self.try_clean();

		let was_unsubscribed = {
			let mut state = self.deferred_state.lock_ignore_poison();
			let was_unsubscribed = state.is_unsubscribed();
			state.observed_unsubscribe = true;
			was_unsubscribed
		};

		if !was_unsubscribed && let Err(_unsubscribe_error) = self.subscribers.try_unsubscribe() {
			self.deferred_state
				.lock_ignore_poison()
				.defer_notification(MulticastNotification::Unsubscribe);
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
		let deferred_state = Arc::new(Mutex::new(MulticastDeferredState::default()));
		let subscriber_id_generator =
			Arc::new(Mutex::new(MulticastSubscriberIdGenerator::default()));

		Self {
			subscribers: SharedSubscribers::new(deferred_state.clone(), subscriber_id_generator),
			deferred_state,
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
