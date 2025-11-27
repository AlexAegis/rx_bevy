use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_operator_composite::CompositeSubscriber;
use rx_core_operator_lift_option::LiftOptionSubscriber;
use rx_core_operator_map::MapSubscriber;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

pub type FilterMapSubscriber<In, InError, Mapper, Out, Destination> = CompositeSubscriber<
	MapSubscriber<In, InError, Mapper, Option<Out>, LiftOptionSubscriber<Destination>>,
	Destination,
>;

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct FilterMapOperator<In, InError, Mapper, Out, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	mapper: Mapper,
	_phantom_data: PhantomData<(In, Out, InError, Context)>,
}

impl<In, InError, Mapper, Out, Context> FilterMapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
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

impl<In, InError, Mapper, Out, Context> Operator
	for FilterMapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= FilterMapSubscriber<In, InError, Mapper, Out, Destination>
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
		CompositeSubscriber::new(MapSubscriber::new(
			LiftOptionSubscriber::new(destination),
			self.mapper.clone(),
		))
	}
}
