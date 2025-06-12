use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, Operator, Subscriber};

#[derive(Debug)]
pub struct FinalizeOperator<In, Callback, Error>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Callback, Error> Operator for FinalizeOperator<In, Callback, Error>
where
	Callback: Clone + FnOnce(),
{
	type Fw = FinalizeOperatorForwarder<In, Callback, Error>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as ObservableOutput>::Out,
				Error = <Self::Fw as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(
			destination,
			FinalizeOperatorForwarder {
				_phantom_data: PhantomData,
				callback: Some(self.callback.clone()),
			},
		)
	}
}

pub struct FinalizeOperatorForwarder<In, Callback, Error>
where
	Callback: FnOnce(),
{
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Callback, Error> ObservableOutput for FinalizeOperatorForwarder<In, Callback, Error>
where
	Callback: FnOnce(),
{
	type Out = In;
	type OutError = Error;
}

impl<In, Callback, Error> Forwarder for FinalizeOperatorForwarder<In, Callback, Error>
where
	Callback: FnOnce(),
{
	type In = In;
	type InError = Error;

	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.next(next);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		if let Some(complete) = self.callback.take() {
			(complete)();
		}
		destination.complete();
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
		}
	}
}
