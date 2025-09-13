use std::marker::PhantomData;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, ObservableOutput, Observer, ObserverInput, Operation,
	SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike, Tick,
};

pub struct FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	destination: Destination,
	filter: Filter,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter, Destination> FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
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

impl<In, InError, Filter, Destination> SignalContext
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type Context = Destination::Context;
}

impl<In, InError, Filter, Destination> Observer
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if (self.filter)(&next) {
			self.destination.next(next, context);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Filter, Destination> SubscriptionLike
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, Filter, Destination> SubscriptionCollection
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<In, InError, Filter, Destination> ObserverInput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter, Destination> ObservableOutput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Filter, Destination> Operation
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}

impl<In, InError, Filter, Destination> Drop for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Subscriber<In = In, InError = InError>,
{
	fn drop(&mut self) {
		self.assert_closed_when_dropped();
	}
}
