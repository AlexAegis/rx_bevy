use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber,
	context::SubscriptionContext,
};

use crate::TryCaptureSubscriber;

/// The [TryCaptureOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed to not reach downstream.
pub struct TryCaptureOperator<In, InError, Context = ()>
where
	InError: SignalBound,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for TryCaptureOperator<In, InError, Context>
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

impl<In, InError, Context> Operator for TryCaptureOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TryCaptureSubscriber<In, InError, Destination>
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
		TryCaptureSubscriber::new(destination)
	}
}

impl<In, InError, Context> ObserverInput for TryCaptureOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for TryCaptureOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = Result<In, InError>;
	type OutError = ();
}

impl<In, InError, Context> Clone for TryCaptureOperator<In, InError, Context>
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
