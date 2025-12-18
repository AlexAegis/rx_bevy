use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: Signal,
	Destination: Subscriber<In = Out, InError = InError>,
{
	#[destination]
	destination: Destination,
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out, Destination> MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: Signal,
	Destination: Subscriber<In = Out, InError = InError>,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination,
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Destination> Observer
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: Signal,
	Destination: Subscriber<In = Out, InError = InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let mapped = (self.mapper)(next);
		self.destination.next(mapped);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
