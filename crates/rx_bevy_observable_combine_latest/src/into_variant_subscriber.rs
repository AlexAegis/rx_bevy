use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	mapper: Mapper,
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
	IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, mapper: Mapper, error_mapper: ErrorMapper) -> Self {
		Self {
			destination,
			mapper,
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination> Observer
	for IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let mapped = (self.mapper)(next);
		self.destination.next(mapped);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		let mapped_error = (self.error_mapper)(error);

		self.destination.error(mapped_error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination> SubscriptionLike
	for IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination> ObserverInput
	for IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination> ObservableOutput
	for IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = OutError;
}

impl<In, InError, Mapper, ErrorMapper, Out, OutError, Destination> Operation
	for IntoVariantSubscriber<In, InError, Mapper, ErrorMapper, Out, OutError, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	ErrorMapper: Fn(InError) -> OutError,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}
