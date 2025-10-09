use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber, SubscriptionCollection,
};

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback, Context = ()>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Callback, Context> FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
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
	Callback: 'static + Clone + FnOnce(&mut Context),
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= Destination
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection,
	{
		destination.add_fn(self.callback.clone(), context);
		destination
	}
}

impl<In, InError, Callback, Context> ObservableOutput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Context> ObserverInput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Context> Clone for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}
