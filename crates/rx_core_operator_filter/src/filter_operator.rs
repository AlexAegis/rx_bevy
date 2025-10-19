use std::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::FilterSubscriber;

pub struct FilterOperator<In, InError, Filter, Context = ()> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Filter, Context> FilterOperator<In, InError, Filter, Context> {
	pub fn new(filter: Filter) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter, Context> Operator for FilterOperator<In, InError, Filter, Context>
where
	Filter: 'static + for<'a> Fn(&'a In) -> bool + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= FilterSubscriber<In, InError, Filter, Destination>
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

impl<In, InError, Filter, Context> ObserverInput for FilterOperator<In, InError, Filter, Context>
where
	Filter: for<'a> Fn(&'a In) -> bool,
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter, Context> ObservableOutput for FilterOperator<In, InError, Filter, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Filter, Context> Clone for FilterOperator<In, InError, Filter, Context>
where
	Filter: Clone,
{
	fn clone(&self) -> Self {
		Self {
			filter: self.filter.clone(),
			_phantom_data: PhantomData,
		}
	}
}
