use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, Forwarder, ObservableOutput, Observer, ObserverInput, Operator,
	Subscriber, Subscription,
};

#[derive(Debug)]
pub struct IdentityOperator<In, InError> {
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for IdentityOperator<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Clone for IdentityOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableOutput for IdentityOperator<In, InError> {
	type Out = In;
	type OutError = InError;
}

impl<In, InError> ObserverInput for IdentityOperator<In, InError> {
	type In = In;
	type InError = InError;
}

impl<In, InError> Operator for IdentityOperator<In, InError> {
	type Subscriber<Destination: Observer<In = Self::Out, InError = Self::OutError>> =
		IdentitySubscriber<In, InError, Destination>;

	fn operator_subscribe<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		IdentitySubscriber::new(destination)
	}
}

#[derive(Debug)]
pub struct IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer,
{
	fn new(destination: Destination) -> Self {
		Self {
			destination: ClosableDestination::new(destination),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> ObservableOutput for IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> ObserverInput for IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> Observer for IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Destination> Subscription for IdentitySubscriber<In, InError, Destination>
where
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

impl<In, InError, Destination> Subscriber for IdentitySubscriber<In, InError, Destination>
where
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}
