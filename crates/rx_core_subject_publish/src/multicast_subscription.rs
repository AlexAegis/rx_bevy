use std::sync::{Arc, Mutex};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	LockWithPoisonBehavior, Signal, Subscriber, SubscriptionLike, Teardown, TeardownCollection,
};

#[derive(RxSubscription)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	subscriber: Option<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>,
}

impl<In, InError> MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(shared_subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>) -> Self {
		Self {
			subscriber: Some(shared_subscriber),
		}
	}

	pub fn new_closed() -> Self {
		Self { subscriber: None }
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
		if let Some(subscriber) = self.subscriber.take() {
			let mut destination = subscriber.lock_ignore_poison();
			if !destination.is_closed() {
				destination.unsubscribe();
			}
		}
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
