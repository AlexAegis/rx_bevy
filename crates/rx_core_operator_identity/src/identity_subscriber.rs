use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::Subscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
}

impl<Destination> IdentitySubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}
