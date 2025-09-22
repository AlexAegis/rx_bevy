use std::marker::PhantomData;

use rx_bevy_core::{DropContext, ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::FinalizeSubscriber;

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback, Context = ()>
where
	Callback: FnOnce(),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Callback, Context> FinalizeOperator<In, InError, Callback, Context>
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

impl<In, InError, Callback, Context> Operator for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(),
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= FinalizeSubscriber<In, InError, Callback, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		FinalizeSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback, Context> ObservableOutput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: FnOnce(),
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Context> ObserverInput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: FnOnce(),
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Context> Clone for FinalizeOperator<In, InError, Callback, Context>
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
