use core::marker::PhantomData;

use rx_core_traits::{
	Never, ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::ErrorBoundarySubscriber;

/// # [ErrorBoundaryOperator]
///
/// The [ErrorBoundaryOperator] does nothing but ensure that the error type is
/// [Never], giving you a compile error if the upstream error type has
/// accidentally changed to something else, when you want to ensure the
/// downstream error type stays [Never].
#[derive(Debug)]
pub struct ErrorBoundaryOperator<In, Context> {
	_phantom_data: PhantomData<(In, fn(Context))>,
}

impl<In, Context> Default for ErrorBoundaryOperator<In, Context> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Context> Clone for ErrorBoundaryOperator<In, Context> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Context> ObservableOutput for ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
{
	type Out = In;
	type OutError = Never;
}

impl<In, Context> ObserverInput for ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
{
	type In = In;
	type InError = Never;
}

impl<In, Context> Operator for ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= ErrorBoundarySubscriber<Destination>
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
		ErrorBoundarySubscriber::new(destination)
	}
}
