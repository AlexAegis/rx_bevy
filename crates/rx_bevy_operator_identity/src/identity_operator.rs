use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber,
	context::SubscriptionContext,
};

use crate::IdentitySubscriber;

/// # [IdentityOperator]
///
/// The [IdentityOperator] does nothing. It's only purpose is to let you
/// easily define input types for a [CompositeOperator]
#[derive(Debug)]
pub struct IdentityOperator<In, InError, Context> {
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for IdentityOperator<In, InError, Context> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Clone for IdentityOperator<In, InError, Context> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> ObservableOutput for IdentityOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> ObserverInput for IdentityOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Operator for IdentityOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext + 'static,
{
	type Context = Context;
	type Subscriber<Destination>
		= IdentitySubscriber<Destination>
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
		IdentitySubscriber::new(destination)
	}
}
