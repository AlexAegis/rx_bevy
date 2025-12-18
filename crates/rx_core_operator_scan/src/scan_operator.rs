use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Signal, Subscriber};

use crate::ScanSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
pub struct ScanOperator<In, InError, Reducer, Out = In>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
{
	reducer: Reducer,
	seed: Out,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Reducer, Out> ScanOperator<In, InError, Reducer, Out>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
{
	pub fn new(reducer: Reducer, seed: Out) -> Self {
		Self {
			seed,
			reducer,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Reducer, Out> ComposableOperator for ScanOperator<In, InError, Reducer, Out>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
{
	type Subscriber<Destination>
		= ScanSubscriber<In, InError, Reducer, Out, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		ScanSubscriber::new(destination, self.reducer.clone(), self.seed.clone())
	}
}

impl<In, InError, Reducer, Out> Clone for ScanOperator<In, InError, Reducer, Out>
where
	In: Signal,
	InError: Signal,
	Reducer: 'static + Fn(&Out, In) -> Out + Clone + Send + Sync,
	Out: Signal + Clone,
{
	fn clone(&self) -> Self {
		Self {
			seed: self.seed.clone(),
			reducer: self.reducer.clone(),
			_phantom_data: PhantomData,
		}
	}
}
