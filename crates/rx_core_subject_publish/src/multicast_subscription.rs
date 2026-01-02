use std::sync::{Arc, Mutex};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	LockWithPoisonBehavior, Never, Signal, Subscriber, SubscriptionLike, Teardown,
	TeardownCollection,
};

use crate::internal::{
	MulticastDeferredState, MulticastNotification, MulticastSubscriberId, SharedSubscribers,
};

#[derive(RxSubscription)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct MulticastSubscription<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	id: Option<MulticastSubscriberId>,
	subscribers: SharedSubscribers<In, InError>,
	state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
	subscriber: Option<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>,
}

impl<In, InError> MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub(crate) fn new(
		id: MulticastSubscriberId,
		state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
		subscribers: SharedSubscribers<In, InError>,
		shared_subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>,
	) -> Self {
		Self {
			id: Some(id),
			subscribers,
			state,
			subscriber: Some(shared_subscriber),
		}
	}

	pub(crate) fn new_closed(
		state: Arc<Mutex<MulticastDeferredState<In, InError>>>,
		subscribers: SharedSubscribers<In, InError>,
	) -> Self {
		Self {
			id: None,
			state,
			subscribers,
			subscriber: None,
		}
	}

	fn try_check_is_closed(&self) -> bool {
		if let Some(subscriber) = self.subscriber.as_ref() {
			subscriber
				.try_lock()
				.map(|s| s.is_closed())
				.unwrap_or(false)
		} else {
			true
		}
	}
}

impl<In, InError> SubscriptionLike for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn is_closed(&self) -> bool {
		if let Some(subscriber) = self.subscriber.as_ref() {
			subscriber.is_closed()
		} else {
			true
		}
	}

	fn unsubscribe(&mut self) {
		if !self.try_check_is_closed()
			&& let Some(id) = self.id
			&& let Err(_unsubscribe_error) = self.subscribers.try_unsubscribe_by_id(id)
		{
			self.state
				.lock_ignore_poison()
				.defer_notification(MulticastNotification::UnsubscribeById(id));
		}

		self.subscriber.take();
	}
}

impl<In, InError> TeardownCollection for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if let Some(subscriber) = &mut self.subscriber {
			subscriber.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl<In, InError> Drop for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
