use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, ObservableOutput, ObserverInput, Operator, Subscriber, SubscriptionCollection,
};

use crate::TapSubscriber;

#[derive(Debug)]
pub struct TapOperator<In, InError, Callback, Context = ()>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Callback, Context> TapOperator<In, InError, Callback, Context>
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

impl<In, InError, Callback, Context> Operator for TapOperator<In, InError, Callback, Context>
where
	Callback: 'static + Clone + for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TapSubscriber<In, InError, Callback, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection,
	{
		TapSubscriber::new(destination, self.callback.clone())
	}
}

impl<In, InError, Callback, Context> ObservableOutput
	for TapOperator<In, InError, Callback, Context>
where
	Callback: for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Context> ObserverInput for TapOperator<In, InError, Callback, Context>
where
	Callback: for<'a> Fn(&'a In),
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Context> Clone for TapOperator<In, InError, Callback, Context>
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
