use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

pub struct TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	destination: Destination,
	callback: OnNext,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, Destination> TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	pub fn new(destination: Destination, callback: OnNext) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Destination> SignalContext
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type Context = Destination::Context;
}

impl<In, InError, OnNext, Destination> Observer
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		(self.callback)(&next, context);
		self.destination.next(next, context);
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

impl<In, InError, OnNext, Destination> SubscriptionLike
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
	Destination: SubscriptionLike,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<In, InError, OnNext, Destination> SubscriptionCollection
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<In, InError, OnNext, Destination> ObservableOutput
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, OnNext, Destination> ObserverInput
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + for<'a> Fn(&'a In, &'a mut Destination::Context),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}
