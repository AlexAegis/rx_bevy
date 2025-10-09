use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber, SubscriptionCollection,
};

use crate::TakeSubscriber;

#[derive(Debug)]
pub struct TakeOperator<In, InError, Context = ()> {
	pub count: usize,
	pub _phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> TakeOperator<In, InError, Context> {
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for TakeOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TakeSubscriber<In, InError, Destination>
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
		TakeSubscriber::new(destination, self.count)
	}
}

impl<In, InError, Context> ObserverInput for TakeOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for TakeOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Clone for TakeOperator<In, InError, Context> {
	fn clone(&self) -> Self {
		Self {
			count: self.count,
			_phantom_data: PhantomData,
		}
	}
}
