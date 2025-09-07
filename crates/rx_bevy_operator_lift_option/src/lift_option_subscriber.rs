use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

pub struct LiftOptionSubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> LiftOptionSubscriber<In, InError, Destination>
where
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> SignalContext for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Destination> Observer for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		if let Some(next) = next {
			self.destination.next(next, context);
		}
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Destination> SubscriptionLike for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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
	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, Destination> SubscriptionCollection
	for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
		self.destination.add(subscription, context);
	}
}

impl<In, InError, Destination> ObserverInput for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = Option<In>;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> Operation for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
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
