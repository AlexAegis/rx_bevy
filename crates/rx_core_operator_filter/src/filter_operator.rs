use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::FilterSubscriber;

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct FilterOperator<In, InError, Filter>
where
	In: Signal,
	InError: Signal,
	Filter: 'static + for<'a> Fn(&'a In, usize) -> bool + Clone + Send + Sync,
{
	filter: Filter,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, Filter> FilterOperator<In, InError, Filter>
where
	In: Signal,
	InError: Signal,
	Filter: 'static + for<'a> Fn(&'a In, usize) -> bool + Clone + Send + Sync,
{
	pub fn new(filter: Filter) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter> ComposableOperator for FilterOperator<In, InError, Filter>
where
	In: Signal,
	InError: Signal,
	Filter: 'static + for<'a> Fn(&'a In, usize) -> bool + Clone + Send + Sync,
{
	type Subscriber<Destination>
		= FilterSubscriber<Filter, Destination>
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
		FilterSubscriber::new(destination, self.filter.clone())
	}
}
