use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operator, Subscriber,
	Subscription,
};

#[derive(Debug)]
pub struct TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> Operator for TapOperator<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type Subscriber<D: Observer<In = Self::Out, InError = Self::OutError>> =
		TapSubscriber<In, InError, Callback, D>;

	fn operator_subscribe<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		TapSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback> ObservableOutput for TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback> TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback> Clone for TapOperator<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}

pub struct TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	destination: ClosableDestination<Destination>,
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination: ClosableDestination::new(destination),
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> ObservableOutput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Observer
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		(self.callback)(&next);
		self.destination.next(next);
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

impl<In, InError, Callback, Destination> Subscriber
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	type Destination = Destination;
}

impl<In, InError, Callback, Destination> Subscription
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}
