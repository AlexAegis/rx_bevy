use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber, SubscriptionCollection,
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
	Filter: 'static + Clone + for<'a> Fn(&'a In) -> bool,
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= FilterSubscriber<In, InError, Filter, Destination>
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
		FilterSubscriber::new(destination, self.filter.clone())
	}
}

impl<In, InError, Filter, Context> ObserverInput for FilterOperator<In, InError, Filter, Context>
where
	Filter: for<'a> Fn(&'a In) -> bool,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter, Context> ObservableOutput for FilterOperator<In, InError, Filter, Context>
where
	In: 'static,
	InError: 'static,
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
