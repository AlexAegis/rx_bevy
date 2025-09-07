use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

pub struct MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Out, OutError, Destination>
	MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, OutError, Destination> SignalContext
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Out, OutError, Destination> Observer
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		self.destination.next(next.into(), context);
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination.error(error.into(), context);
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

impl<In, InError, Out, OutError, Destination> SubscriptionLike
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
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

impl<In, InError, Out, OutError, Destination> SubscriptionCollection
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
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

impl<In, InError, Out, OutError, Destination> ObserverInput
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Out, OutError, Destination> ObservableOutput
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = OutError;
}

impl<In, InError, Out, OutError, Destination> Operation
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
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
