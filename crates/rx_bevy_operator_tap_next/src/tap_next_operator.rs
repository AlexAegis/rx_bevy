use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber,
	context::SubscriptionContext,
};

use crate::TapNextSubscriber;

#[derive(Debug)]
pub struct TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context),
{
	on_next: OnNext,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, OnNext, Context> TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context),
{
	pub fn new(on_next: OnNext) -> Self {
		Self {
			on_next,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Context> Operator for TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context) + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TapNextSubscriber<In, InError, OnNext, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		TapNextSubscriber::new(destination, self.on_next.clone())
	}
}

impl<In, InError, OnNext, Context> ObservableOutput
	for TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context) + Clone,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, OnNext, Context> ObserverInput for TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context) + Clone,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnNext, Context> Clone for TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Context) + Clone,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			on_next: self.on_next.clone(),
			_phantom_data: PhantomData,
		}
	}
}
