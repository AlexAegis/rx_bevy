use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Operator, Subscriber, SubscriptionLike,
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
	Callback: 'static + Clone + for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
{
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		TapSubscriber<In, InError, Callback, D>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		TapSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback> ObservableOutput for TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
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
	destination: Destination,
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
			destination,
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
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Observer
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
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

impl<In, InError, Callback, Destination> Operation
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	type Destination = Destination;
}

impl<In, InError, Callback, Destination> SubscriptionLike
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Subscriber<In = In, InError = InError>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}
