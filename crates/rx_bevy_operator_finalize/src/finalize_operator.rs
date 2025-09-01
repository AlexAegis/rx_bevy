use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::FinalizeSubscriber;

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
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

impl<In, InError, Callback> Operator for FinalizeOperator<In, InError, Callback>
where
	Callback: 'static + Clone + FnOnce(),
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		FinalizeSubscriber<In, InError, Callback, Destination>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		FinalizeSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback> ObservableOutput for FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback> ObserverInput for FinalizeOperator<In, InError, Callback>
where
	Callback: FnOnce(),
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
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
