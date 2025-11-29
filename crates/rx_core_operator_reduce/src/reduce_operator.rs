use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber, SubscriptionContext};

use crate::ReduceSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct ReduceOperator<In, InError, Reducer, Out = In, Context = ()>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
	Context: SubscriptionContext,
{
	reducer: Reducer,
	seed: Out,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Reducer, Out, Context> ReduceOperator<In, InError, Reducer, Out, Context>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
	Context: SubscriptionContext,
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
	for ReduceOperator<In, InError, Reducer, Out, Context>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= ReduceSubscriber<In, InError, Reducer, Out, Destination>
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
		ReduceSubscriber::new(destination, self.reducer.clone(), self.seed.clone())
	}
}

impl<In, InError, Reducer, Out, Context> Clone
	for ReduceOperator<In, InError, Reducer, Out, Context>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			seed: self.seed.clone(),
			reducer: self.reducer.clone(),
			_phantom_data: PhantomData,
		}
	}
}
