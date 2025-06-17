use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operation, Operator,
	Subscriber, Subscription,
};

pub struct FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> ObserverInput for FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
	In: 'static,
	Error: 'static,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error> ObservableOutput for FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
	Out: 'static,
	Error: 'static,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error> Operator for FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: 'static + Clone + Fn(In) -> Option<Out>,
	In: 'static,
	Out: 'static,
	Error: 'static,
{
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		FilterMapSubscriber<Mapper, In, Out, Error, D>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		FilterMapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<Mapper, In, Out, Error> FilterMapOperator<Mapper, In, Out, Error>
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

impl<Mapper, In, Out, Error> Clone for FilterMapOperator<Mapper, In, Out, Error>
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

pub struct FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	pub mapper: Mapper,
	pub index: u32,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error, Destination> FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer,
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

impl<Mapper, In, Out, Error, Destination> ObserverInput
	for FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	Out: 'static,
	Error: 'static,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error, Destination> ObservableOutput
	for FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer,
	Out: 'static,
	Error: 'static,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error, Destination> Observer
	for FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	Out: 'static,
	Error: 'static,
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

impl<Mapper, In, Out, Error, Destination> Subscription
	for FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Out: 'static,
	Error: 'static,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<Mapper, In, Out, Error, Destination> Operation
	for FilterMapSubscriber<Mapper, In, Out, Error, Destination>
where
	Mapper: Fn(In) -> Option<Out>,
	Destination: Observer<In = Out, InError = Error>,
{
	type Destination = Destination;
}
