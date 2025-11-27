use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::FilterSubscriber;

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct FilterOperator<In, InError, Filter, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: 'static + for<'a> Fn(&'a In) -> bool + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	filter: Filter,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Filter, Context> FilterOperator<In, InError, Filter, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: 'static + for<'a> Fn(&'a In) -> bool + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	pub fn new(filter: Filter) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter, Context> Operator for FilterOperator<In, InError, Filter, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: 'static + for<'a> Fn(&'a In) -> bool + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= FilterSubscriber<Filter, Destination>
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
		FilterSubscriber::new(destination, self.filter.clone())
	}
}
