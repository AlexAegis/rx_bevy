use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionCollection,
	context::SubscriptionContext,
};

#[derive(Debug)]
pub struct FinalizeOperator<In, InError, Callback, Context = ()>
where
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Callback, Context> FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
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
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= Destination
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		destination.add_fn(self.callback.clone(), context);
		destination
	}
}

impl<In, InError, Callback, Context> ObservableOutput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Context> ObserverInput
	for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Context> Clone for FinalizeOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context) + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}
