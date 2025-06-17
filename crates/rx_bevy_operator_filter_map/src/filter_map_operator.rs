use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Operator, Subscriber, Subscription,
};

pub struct FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, InError)>,
}

impl<In, InError, Mapper, Out> ObserverInput for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out> ObservableOutput for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out> Operator for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: 'static + Clone + Fn(In) -> Option<Out>,
	In: 'static,
	Out: 'static,
	InError: 'static,
{
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		FilterMapSubscriber<In, InError, Mapper, Out, D>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		FilterMapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, Out> FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out> Clone for FilterMapOperator<In, InError, Mapper, Out>
where
	Mapper: Clone + Fn(In) -> Option<Out>,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}

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

impl<In, InError, Mapper, Out, Destination> Subscription
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

impl<In, InError, Mapper, Out, Destination> Operation
	for FilterMapSubscriber<In, InError, Mapper, Out, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Subscriber<In = Out, InError = InError>,
{
	type Destination = Destination;
}
