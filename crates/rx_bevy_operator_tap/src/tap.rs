use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverConnector};
use rx_bevy_operator::{ForwardObserver, Operator};

#[derive(Debug)]
pub struct TapOperator<In, Callback, Error>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
	_phantom_data_error: PhantomData<Error>,
}

impl<In, Callback, Error> Operator for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;
	type InError = Error;
	type OutError = Error;

	type InternalSubscriber = Self;

	fn operator_subscribe<
		Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> rx_bevy_operator::ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(self.clone(), destination)
	}
}

impl<In, Callback, Error> ObserverConnector for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;
	type InError = Error;
	type OutError = Error;

	fn push_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.on_push(next);
	}

	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.on_error(error);
	}

	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		destination.on_complete();
	}
}

impl<In, Callback, Error> TapOperator<In, Callback, Error>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
			_phantom_data_error: PhantomData,
		}
	}
}

impl<In, Callback, Error> Clone for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
			_phantom_data_error: PhantomData,
		}
	}
}
