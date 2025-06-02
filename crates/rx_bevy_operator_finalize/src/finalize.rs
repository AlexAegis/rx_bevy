use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverConnector};
use rx_bevy_operator::{ForwardObserver, Operator};

#[derive(Debug)]
pub struct FinalizeOperator<In, Callback, Error>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
	_phantom_data_error: PhantomData<Error>,
}

impl<In, Callback, Error> Operator for FinalizeOperator<In, Callback, Error>
where
	Callback: Clone + FnOnce(),
{
	type In = In;
	type Out = In;
	type InError = Error;
	type OutError = Error;

	type InternalSubscriber = FinalizeOperatorInstance<In, Callback, Error>;

	fn operator_subscribe<
		Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> rx_bevy_operator::ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(
			FinalizeOperatorInstance {
				_phantom_data: PhantomData,
				callback: Some(self.callback.clone()),
			},
			destination,
		)
	}
}

pub struct FinalizeOperatorInstance<In, Callback, Error>
where
	Callback: FnOnce(),
{
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Callback, Error> ObserverConnector for FinalizeOperatorInstance<In, Callback, Error>
where
	Callback: FnOnce(),
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
		if let Some(on_complete) = self.callback.take() {
			(on_complete)();
		}
		destination.on_complete();
	}
}

impl<In, Callback, Error> FinalizeOperator<In, Callback, Error>
where
	Callback: FnOnce(),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
			_phantom_data_error: PhantomData,
		}
	}
}

impl<In, Callback, Error> Clone for FinalizeOperator<In, Callback, Error>
where
	Callback: Clone + FnOnce(),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
			_phantom_data_error: PhantomData,
		}
	}
}
