use std::marker::PhantomData;

use rx_bevy_observable::{
	Forwarder, ObservableOutput, Observer, ObserverInput, Operator, Subscriber,
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
	type Fw = TapForwarder<In, InError, Callback>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(
			destination,
			TapForwarder {
				callback: self.callback.clone(),
				_phantom_data: PhantomData,
			},
		)
	}
}

pub struct TapForwarder<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
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
