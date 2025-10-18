use std::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber,
	context::SubscriptionContext,
};

use crate::TapNextSubscriber;

#[derive(Debug)]
pub struct TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	on_next: OnNext,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, OnNext, Context> TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
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
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
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
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
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
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, OnNext, Context> ObserverInput for TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnNext, Context> Clone for TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			on_next: self.on_next.clone(),
			_phantom_data: PhantomData,
		}
	}
}
