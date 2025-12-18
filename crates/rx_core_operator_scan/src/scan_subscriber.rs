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
pub struct ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Reducer: Fn(&Out, In) -> Out + Send + Sync,
	Out: Signal + Clone,
	Destination: Subscriber<In = Out, InError = InError>,
{
	#[destination]
	destination: Destination,
	accumulator: Out,
	reducer: Reducer,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Reducer, Out, Destination> ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Reducer: Fn(&Out, In) -> Out + Send + Sync,
	Out: Signal + Clone,
	Destination: Subscriber<In = Out, InError = InError>,
{
	pub fn new(destination: Destination, reducer: Reducer, seed: Out) -> Self {
		Self {
			accumulator: seed,
			destination,
			reducer,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Reducer, Out, Destination> Observer
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: Signal,
	InError: Signal,
	Reducer: Fn(&Out, In) -> Out + Send + Sync,
	Out: Signal + Clone,
	Destination: Subscriber<In = Out, InError = InError>,
{
	fn next(&mut self, next: Self::In) {
		self.accumulator = (self.reducer)(&self.accumulator, next);
		self.destination.next(self.accumulator.clone());
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
