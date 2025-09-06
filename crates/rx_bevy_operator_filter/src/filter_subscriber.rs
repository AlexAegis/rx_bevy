use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

pub struct FilterSubscriber<In, InError, Filter, Destination>
where
	Destination: Observer,
{
	destination: Destination,
	filter: Filter,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter, Destination> FilterSubscriber<In, InError, Filter, Destination>
where
	Destination: Observer,
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
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Filter, Destination> Observer
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
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
	Destination: Observer,
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
	Destination: Observer,
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
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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
