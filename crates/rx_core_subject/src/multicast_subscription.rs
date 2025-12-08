use std::sync::{Arc, Mutex};

use rx_core_traits::{
	Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike, Teardown, TeardownCollection,
};

/// This Subscription extends a shared subscriber into a clone-able subscription
/// To be a proper subscription it must also implement [Default] in order to be
/// used in contexts (combinator observables like [ZipObservable] and [CombineLatestObservable]) where multiple subscriptions has to be wrapped in one
pub struct MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	closed_flag: SubscriptionClosedFlag,
	subscriber: Option<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>,
}

impl<In, InError> MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(shared_subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>) -> Self {
		Self {
			closed_flag: shared_subscriber.is_closed().into(),
			subscriber: Some(shared_subscriber),
		}
	}

	pub fn new_closed() -> Self {
		Self {
			closed_flag: true.into(),
			subscriber: None,
		}
	}
}

impl<In, InError> Default for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			closed_flag: false.into(),
			subscriber: None,
		}
	}
}

impl<In, InError> Clone for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn clone(&self) -> Self {
		Self {
			closed_flag: self.closed_flag.clone(),
			subscriber: self.subscriber.clone(),
		}
	}
}

impl<In, InError> SubscriptionLike for MulticastSubscription<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();
			if let Some(mut subscriber) = self.subscriber.take() {
				subscriber.unsubscribe();
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
		if !self.is_closed()
			&& let Some(subscriber) = &mut self.subscriber
		{
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
		if !self.is_closed() {
			self.unsubscribe();
		}
		// Does not unsubscribe the subscriber on drop as it is shared.
		// Only the teardown is unsubscribed which is local to the reference instance
	}
}
