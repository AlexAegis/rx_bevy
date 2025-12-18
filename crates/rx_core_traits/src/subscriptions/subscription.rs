use rx_core_macro_subscription_derive::RxSubscription;

use crate::{Subscriber, SubscriptionData};

#[derive(RxSubscription)]
#[_rx_core_traits_crate(crate)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct Subscription<Destination>
where
	Destination: Subscriber,
{
	#[teardown]
	teardown: SubscriptionData,
	#[destination]
	destination: Destination,
}

impl<Destination> Subscription<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			teardown: SubscriptionData::default(),
		}
	}
}
