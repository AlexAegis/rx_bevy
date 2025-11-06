use core::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber,
	SignalBound, Subscriber, SubscriptionContext, SubscriptionLike, Teardown, Tick, Tickable,
	WithPrimaryCategory, WithSubscriptionContext,
};

pub struct FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	destination: Destination,
	filter: Filter,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter, Destination> FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	pub fn new(destination: Destination, filter: Filter) -> Self {
		Self {
			destination,
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter, Destination> WithSubscriptionContext
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type Context = Destination::Context;
}

impl<In, InError, Filter, Destination> WithPrimaryCategory
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError, Filter, Destination> ObserverUpgradesToSelf
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
}

impl<In, InError, Filter, Destination> Observer
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if (self.filter)(&next) {
			self.destination.next(next, context);
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Filter, Destination> Tickable
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Filter, Destination> SubscriptionLike
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<In, InError, Filter, Destination> ObserverInput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter, Destination> ObservableOutput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type Out = In;
	type OutError = InError;
}
