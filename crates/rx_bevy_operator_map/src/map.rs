use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operation, Operator,
	Subscriber, Subscription,
};

pub struct MapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> ObservableOutput for MapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error> ObserverInput for MapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error> Operator for MapOperator<Mapper, In, Out, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>> =
		MapSubscriber<Mapper, In, Out, Error, Destination>;

	fn operator_subscribe<
		Destination: Subscriber<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<Mapper, In, Out, Error> MapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<Mapper, In, Out, Error> Clone for MapOperator<Mapper, In, Out, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}

pub struct MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	mapper: Mapper,
	index: u32,
	_phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error, Destination> MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination: ClosableDestination::new(destination),
			mapper,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<Mapper, In, Out, Error, Destination> ObservableOutput
	for MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error, Destination> ObserverInput
	for MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error, Destination> Observer
	for MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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

impl<Mapper, In, Out, Error, Destination> Subscription
	for MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<Mapper, In, Out, Error, Destination> Operation
	for MapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}
