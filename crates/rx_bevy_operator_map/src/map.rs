use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operation, Operator,
	Subscriber, Subscription,
};

pub struct MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, InError, Out)>,
}

impl<Mapper, In, InError, Out> ObservableOutput for MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<Mapper, In, InError, Out> ObserverInput for MapOperator<Mapper, In, InError, Out>
where
	Mapper: Fn(In) -> Out,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<Mapper, In, InError, Out> Operator for MapOperator<Mapper, In, InError, Out>
where
	Mapper: 'static + Clone + Fn(In) -> Out,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		MapSubscriber<Mapper, In, InError, Out, Destination>;

	fn operator_subscribe<
		Destination: 'static
			+ Subscriber<
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

impl<Mapper, In, InError, Out> MapOperator<Mapper, In, InError, Out>
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

impl<Mapper, In, InError, Out> Clone for MapOperator<Mapper, In, InError, Out>
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

pub struct MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	mapper: Mapper,
	index: u32,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<Mapper, In, InError, Out, Destination> MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Out: 'static,
	InError: 'static,
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

impl<Mapper, In, InError, Out, Destination> ObservableOutput
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer,
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
	Destination: Observer,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<Mapper, In, InError, Out, Destination> Observer
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
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

impl<Mapper, In, InError, Out, Destination> Subscription
	for MapSubscriber<Mapper, In, InError, Out, Destination>
where
	Mapper: Fn(In) -> Out,
	Destination: Observer<
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
