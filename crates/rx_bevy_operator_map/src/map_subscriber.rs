use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber,
{
	destination: Destination,
	mapper: Mapper,
	index: u32,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<Mapper, In, InError, Out, Destination> MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Out: 'static,
	InError: 'static,
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

impl<Mapper, In, InError, Out, Destination> Observer
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
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
		let mapped = (self.mapper)(next);
		self.index += 1;
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

impl<Mapper, In, InError, Out, Destination> SubscriptionLike
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
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

impl<Mapper, In, InError, Out, Destination> ObservableOutput
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<Mapper, In, InError, Out, Destination> ObserverInput
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<Mapper, In, InError, Out, Destination> Operation
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Out: 'static,
	InError: 'static,
{
	type Destination = Destination;
}
