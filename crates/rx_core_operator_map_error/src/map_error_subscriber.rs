use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber, SubscriptionLike};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct MapErrorSubscriber<In, InError, ErrorMapper, OutError, Destination>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: FnOnce(InError) -> OutError + Send + Sync,
	OutError: Signal,
	Destination: Subscriber<In = In, InError = OutError>,
{
	#[destination]
	destination: Destination,
	error_mapper: Option<ErrorMapper>,
	_phantom_data: PhantomData<fn(In, InError, OutError) -> (In, InError, OutError)>,
}

impl<In, InError, ErrorMapper, OutError, Destination>
	MapErrorSubscriber<In, InError, ErrorMapper, OutError, Destination>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: FnOnce(InError) -> OutError + Send + Sync,
	OutError: Signal,
	Destination: Subscriber<In = In, InError = OutError>,
{
	pub fn new(destination: Destination, error_mapper: ErrorMapper) -> Self {
		Self {
			destination,
			error_mapper: Some(error_mapper),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper, OutError, Destination> Observer
	for MapErrorSubscriber<In, InError, ErrorMapper, OutError, Destination>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: FnOnce(InError) -> OutError + Send + Sync,
	OutError: Signal,
	Destination: Subscriber<In = In, InError = OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_closed()
			&& let Some(error_mapper) = self.error_mapper.take()
		{
			let mapped_error = (error_mapper)(error);
			self.destination.error(mapped_error);
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
