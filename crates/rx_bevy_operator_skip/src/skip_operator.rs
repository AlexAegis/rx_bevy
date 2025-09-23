use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, ObservableOutput, ObserverInput, Operator, Subscriber, SubscriptionCollection,
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
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= SkipSubscriber<In, InError, Destination>
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
		SkipSubscriber::new(destination, self.count)
	}
}

impl<In, InError, Context> ObserverInput for SkipOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for SkipOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
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
