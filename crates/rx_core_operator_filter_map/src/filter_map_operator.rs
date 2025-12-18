use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_operator_composite::CompositeSubscriber;
use rx_core_operator_lift_option::LiftOptionSubscriber;
use rx_core_operator_map::MapSubscriber;
use rx_core_traits::{ComposableOperator, Signal, Subscriber};

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
pub struct FilterMapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: Signal,
{
	mapper: Mapper,
	_phantom_data: PhantomData<(In, Out, InError)>,
}

impl<In, InError, Mapper, Out> FilterMapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: Signal,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out> ComposableOperator for FilterMapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: Signal,
{
	type Subscriber<Destination>
		= FilterMapSubscriber<In, InError, Mapper, Out, Destination>
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
		CompositeSubscriber::new(MapSubscriber::new(
			LiftOptionSubscriber::new(destination),
			self.mapper.clone(),
		))
	}
}
