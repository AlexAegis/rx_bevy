use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

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
	type Subscriber = TapForwarder<In, InError, Callback>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.callback.clone())
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

pub struct TapForwarder<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> TapForwarder<In, InError, Callback>
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

impl<In, InError, Callback> ObservableOutput for TapForwarder<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for TapForwarder<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback> Forwarder for TapForwarder<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		(self.callback)(&next);
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
