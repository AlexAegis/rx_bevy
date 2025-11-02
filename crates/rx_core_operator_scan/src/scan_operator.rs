use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_traits::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::ScanSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct ScanOperator<In, InError, Reducer, Out = In, Context = ()>
where
	Reducer: Fn(&Out, In) -> Out,
	Out: Clone,
{
	reducer: Reducer,
	seed: Out,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Reducer, Out, Context> ScanOperator<In, InError, Reducer, Out, Context>
where
	Reducer: Fn(&Out, In) -> Out,
	Out: Clone,
{
	pub fn new(reducer: Reducer, seed: Out) -> Self {
		Self {
			seed,
			reducer,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Reducer, Out, Context> Operator
	for ScanOperator<In, InError, Reducer, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= ScanSubscriber<In, InError, Reducer, Out, Destination>
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
		ScanSubscriber::new(destination, self.reducer.clone(), self.seed.clone())
	}
}

impl<In, InError, Reducer, Out, Context> ObservableOutput
	for ScanOperator<In, InError, Reducer, Out, Context>
where
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound + Clone,
	InError: SignalBound,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Reducer, Out, Context> ObserverInput
	for ScanOperator<In, InError, Reducer, Out, Context>
where
	Reducer: Fn(&Out, In) -> Out,
	Out: Clone,
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Reducer, Out, Context> Clone for ScanOperator<In, InError, Reducer, Out, Context>
where
	Reducer: Clone + Fn(&Out, In) -> Out,
	Out: Clone,
{
	fn clone(&self) -> Self {
		Self {
			seed: self.seed.clone(),
			reducer: self.reducer.clone(),
			_phantom_data: PhantomData,
		}
	}
}
