use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

use crate::{AdsrEnvelopePhase, AdsrEnvelopeState, AdsrOperatorOptions, AdsrSignal};

// TODO: It'd be nice to control the envelope live, I guess that could be done by querying the subscriber itself, but it would be nicer to control the operator itself, in case there are many observers
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	destination: Destination,
	is_getting_activated: bool,
	state: AdsrEnvelopeState,
	pub options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	pub fn new(destination: Destination, options: AdsrOperatorOptions) -> Self {
		Self {
			destination,
			options,
			is_getting_activated: false,
			state: AdsrEnvelopeState::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, Destination> SignalContext for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	type Context = Destination::Context;
}

impl<InError, Destination> Observer for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In, _context: &mut Self::Context) {
		self.is_getting_activated = next;
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
		let next =
			self.state
				.calculate_output(self.options.envelope, self.is_getting_activated, tick);

		if !matches!(next.adsr_envelope_phase, AdsrEnvelopePhase::None) {
			self.destination.next(next, context);
		}
	}
}

impl<InError, Destination> SubscriptionLike for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
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

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<InError, Destination> SubscriptionCollection for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	Destination: SubscriptionCollection,
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

impl<InError, Destination> ObserverInput for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	type In = bool;
	type InError = InError;
}

impl<InError, Destination> ObservableOutput for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	type Out = AdsrSignal;
	type OutError = InError;
}

impl<InError, Destination> Operation for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	type Destination = Destination;
}
