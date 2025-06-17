use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operation, Operator,
	Subscriber, Subscription,
};

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> ObservableOutput for FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback> Operator for FinalizeOperator<In, InError, Callback>
where
	Callback: Clone + FnOnce(),
{
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>> =
		FinalizeSubscriber<In, InError, Callback, Destination>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		FinalizeSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback> FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback> Clone for FinalizeOperator<In, InError, Callback>
where
	Callback: Clone + FnOnce(),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}

pub struct FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	/// It's in an option so it can be removed when used, allowing the use of an FnOnce
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination: ClosableDestination::new(destination),
			callback: Some(callback),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> ObservableOutput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Observer
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
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

	#[inline]
	fn complete(&mut self) {
		if let Some(complete) = self.callback.take() {
			(complete)();
		}
		self.destination.complete();
	}
}

impl<In, InError, Callback, Destination> Subscription
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<In, InError, Callback, Destination> Operation
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}
