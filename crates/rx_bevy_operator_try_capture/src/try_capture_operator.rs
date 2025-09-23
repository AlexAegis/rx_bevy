use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, ObservableOutput, ObserverInput, Operator, Subscriber, SubscriptionCollection,
};

use crate::TryCaptureSubscriber;

/// The [TryCaptureOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed to not reach downstream.
pub struct TryCaptureOperator<In, InError, Context = ()>
where
	InError: 'static,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for TryCaptureOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for TryCaptureOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TryCaptureSubscriber<In, InError, Destination>
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
		TryCaptureSubscriber::new(destination)
	}
}

impl<In, InError, Context> ObserverInput for TryCaptureOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for TryCaptureOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type Out = Result<In, InError>;
	type OutError = ();
}

impl<In, InError, Context> Clone for TryCaptureOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
