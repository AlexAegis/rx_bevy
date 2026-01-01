use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{Signal, Subscriber, subscribers::subscriber_box::BoxedSubscriber};

#[derive_where(Debug)]
#[derive(RxSubscriber)]
#[_rx_core_traits_crate(crate)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip(Debug))]
	#[destination]
	destination: BoxedSubscriber<In, InError>,
}

impl<In, InError> ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new<Destination>(destination: Destination) -> Self
	where
		Destination: 'static + Subscriber<In = In, InError = InError> + Send + Sync,
	{
		Self {
			destination: Box::new(destination),
		}
	}
}
