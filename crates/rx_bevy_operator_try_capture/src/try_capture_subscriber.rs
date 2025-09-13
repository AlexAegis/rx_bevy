use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Tick,
};

pub struct TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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

impl<In, InError, Destination> SignalContext for TryCaptureSubscriber<In, InError, Destination>
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

impl<In, InError, Destination> Observer for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(Ok(next), context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.next(Err(error), context);
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

impl<In, InError, Destination> SubscriptionLike for TryCaptureSubscriber<In, InError, Destination>
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
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, Destination> SubscriptionCollection
	for TryCaptureSubscriber<In, InError, Destination>
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
	fn add<S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<In, InError, Destination> ObserverInput for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for TryCaptureSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = Result<In, InError>;
	type OutError = ();
}

impl<In, InError, Destination> Operation for TryCaptureSubscriber<In, InError, Destination>
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
