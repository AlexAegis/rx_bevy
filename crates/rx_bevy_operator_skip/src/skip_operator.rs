use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalBound, SignalContext, Subscriber,
};

use crate::SkipSubscriber;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// after which it does nothing.
pub struct SkipOperator<In, InError, Context = ()> {
	pub count: usize,
	pub _phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> SkipOperator<In, InError, Context> {
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= SkipSubscriber<In, InError, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context> + Send + Sync,
	{
		SkipSubscriber::new(destination, self.count)
	}
}

impl<In, InError, Context> ObserverInput for SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Clone for SkipOperator<In, InError, Context> {
	fn clone(&self) -> Self {
		Self {
			count: self.count,
			_phantom_data: PhantomData,
		}
	}
}
