use std::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::LiftOptionSubscriber;

pub struct LiftOptionOperator<In, InError, Context = ()> {
	pub _phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for LiftOptionOperator<In, InError, Context> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for LiftOptionOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= LiftOptionSubscriber<In, InError, Destination>
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
		LiftOptionSubscriber::new(destination)
	}
}

impl<In, InError, Context> ObserverInput for LiftOptionOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = Option<In>;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for LiftOptionOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Clone for LiftOptionOperator<In, InError, Context> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
