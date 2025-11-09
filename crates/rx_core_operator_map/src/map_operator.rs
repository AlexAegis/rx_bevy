use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::MapSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct MapOperator<In, InError, Mapper, Out = In, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, InError, Out, Context)>,
}

impl<In, InError, Mapper, Out, Context> MapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Context> Operator for MapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= MapSubscriber<In, InError, Mapper, Out, Destination>
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
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, Out, Context> Clone for MapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
