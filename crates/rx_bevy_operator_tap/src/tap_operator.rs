use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::TapSubscriber;

#[derive(Debug)]
pub struct TapOperator<In, InError, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
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
