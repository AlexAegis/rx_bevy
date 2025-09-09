use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

// TODO: Fix
pub struct FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
{
	destination: Destination,
	/// It's in an option so it can be removed when used, allowing the use of an FnOnce
	callback: Option<Callback>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback, Destination> FinalizeSubscriber<In, InError, Callback, Destination>
where
	In: 'static,
	InError: 'static,
	Callback: 'static + FnOnce(),
	Destination: Subscriber,
{
	pub fn new(destination: Destination, callback: Callback) -> Self {
		Self {
			destination,
			callback: Some(callback),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Destination> SignalContext
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	type Context = Destination::Context;
}

impl<In, InError, Callback, Destination> Observer
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
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
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Some(finalize) = self.callback.take() {
			(finalize)();
		}
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, Callback, Destination> SubscriptionCollection
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
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

impl<In, InError, Callback, Destination> ObservableOutput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Callback, Destination> ObserverInput
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Callback, Destination> Operation
	for FinalizeSubscriber<In, InError, Callback, Destination>
where
	Callback: FnOnce(),
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
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
