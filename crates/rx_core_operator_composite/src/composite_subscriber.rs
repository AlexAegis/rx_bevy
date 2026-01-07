use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{Observer, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[derive_where(Debug; Inner)]
#[rx_in(Inner::In)]
#[rx_in_error(Inner::InError)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	#[destination]
	subscriber: Inner,
	_phantom_data: PhantomData<Destination>,
}

impl<Inner, Destination> CompositeSubscriber<Inner, Destination>
where
	Inner: Subscriber,
	Destination: Observer,
{
	pub fn new(subscriber: Inner) -> Self {
		Self {
			subscriber,
			_phantom_data: PhantomData,
		}
	}
}
