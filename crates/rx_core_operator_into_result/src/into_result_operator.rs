use core::marker::PhantomData;

use rx_core_traits::{
	Never, ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::IntoResultSubscriber;

// TODO: Rename to `into_result`, the `try_` prefix means something that can produce a error result, and the point of this is to avoid that
/// The [IntoResultOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed to not reach downstream.
pub struct IntoResultOperator<In, InError, Context = ()>
where
	InError: SignalBound,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= IntoResultSubscriber<In, InError, Destination>
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
		IntoResultSubscriber::new(destination)
	}
}

impl<In, InError, Context> ObserverInput for IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = Result<In, InError>;
	type OutError = Never;
}

impl<In, InError, Context> Clone for IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
