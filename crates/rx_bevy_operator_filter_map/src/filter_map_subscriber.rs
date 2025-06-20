use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber,
{
	destination: Destination,
	mapper: Mapper,
	index: u32,
	_phantom_data: PhantomData<(In, Out, InError)>,
}

impl<In, InError, Out, Mapper, Destination>
	FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination,
			mapper,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Destination> Observer
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if let Some(mapped) = (self.mapper)(next) {
			self.index += 1;
			self.destination.next(mapped);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Mapper, Out, Destination> SubscriptionLike
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Out: 'static,
	InError: 'static,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<In, InError, Mapper, Out, Destination> ObserverInput
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out, Destination> ObservableOutput
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out, Destination> Operation
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber<In = Out, InError = InError>,
{
	type Destination = Destination;
}
