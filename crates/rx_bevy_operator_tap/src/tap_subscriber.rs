use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

pub struct TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	destination: Destination,
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> TapSubscriber<In, InError, Callback, Destination>
where
	Callback: for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> SignalContext
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type Context = Destination::Context;
}

impl<In, InError, Callback, Destination> Observer
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		(self.callback)(&next);
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

impl<In, InError, Callback, Destination> SubscriptionLike
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Subscriber<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
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

impl<In, InError, Callback, Destination> SubscriptionCollection
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Subscriber<In = In, InError = InError>,
	Destination: SubscriptionCollection,
	In: 'static,
	InError: 'static,
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

impl<In, InError, Callback, Destination> ObservableOutput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Operation
	for TapSubscriber<In, InError, Callback, Destination>
where
	Callback: Clone + for<'a> Fn(&'a In),
	Destination: Observer<In = In, InError = InError>,
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
