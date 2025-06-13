use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> Operator for FinalizeOperator<In, InError, Callback>
where
	Callback: Clone + FnOnce(),
{
	type Fw = FinalizeOperatorForwarder<In, InError, Callback>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.callback.clone())
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

pub struct FinalizeOperatorForwarder<In, InError, Callback>
where
	Callback: FnOnce(),
{
	/// It's in an option so it can be removed when used, allowing the use of an FnOnce
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> FinalizeOperatorForwarder<In, InError, Callback>
where
	Callback: FnOnce(),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback: Some(callback),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback> ObservableOutput for FinalizeOperatorForwarder<In, InError, Callback>
where
	Callback: FnOnce(),
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for FinalizeOperatorForwarder<In, InError, Callback>
where
	Callback: FnOnce(),
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback> Forwarder for FinalizeOperatorForwarder<In, InError, Callback>
where
	Callback: FnOnce(),
{
	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.next(next);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		if let Some(complete) = self.callback.take() {
			(complete)();
		}
		destination.complete();
	}
}
