use rx_core_macro_subscription_derive::RxSubscription;

use crate::{SubscriptionLike, SubscriptionWithTeardown, TeardownCollection};

pub trait EraseSubscriptionExtension {
	fn erase(self) -> ErasedSubscription;
}

impl<S> EraseSubscriptionExtension for S
where
	S: SubscriptionWithTeardown + Send + Sync + 'static,
{
	fn erase(self) -> ErasedSubscription {
		ErasedSubscription::new(self)
	}
}

#[derive(RxSubscription)]
#[_rx_core_common_crate(crate)]
pub struct ErasedSubscription {
	subscription: Box<dyn SubscriptionWithTeardown + Send + Sync + 'static>,
}

impl ErasedSubscription {
	pub fn new<S>(subscription: S) -> Self
	where
		S: SubscriptionWithTeardown + Send + Sync + 'static,
	{
		Self {
			subscription: Box::new(subscription),
		}
	}
}

impl SubscriptionLike for ErasedSubscription {
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.subscription.unsubscribe();
	}
}

impl TeardownCollection for ErasedSubscription {
	fn add_teardown(&mut self, teardown: super::Teardown) {
		self.subscription.add_teardown(teardown);
	}
}
