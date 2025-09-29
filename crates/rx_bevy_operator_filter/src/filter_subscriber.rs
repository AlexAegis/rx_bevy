use std::marker::PhantomData;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, ObservableOutput, Observer, ObserverInput, SignalContext,
	Subscriber, SubscriptionLike, Teardown, Tick,
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

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
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
