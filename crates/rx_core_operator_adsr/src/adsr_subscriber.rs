use core::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, Observer, ObserverInput, PrimaryCategorySubscriber, SignalBound, Subscriber,
	ObserverUpgradesToSelf, SubscriptionContext, SubscriptionLike, Teardown, Tick, Tickable,
	WithPrimaryCategory, WithSubscriptionContext,
};

use crate::{AdsrEnvelopePhase, AdsrEnvelopeState, AdsrSignal, operator::AdsrOperatorOptions};

// TODO: It'd be nice to control the envelope live, I guess that could be done by querying the subscriber itself, but it would be nicer to control the operator itself, in case there are many observers
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	destination: Destination,
	is_getting_activated: bool,
	state: AdsrEnvelopeState,
	pub options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
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

impl<InError, Destination> WithSubscriptionContext for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	type Context = Destination::Context;
}

impl<InError, Destination> WithPrimaryCategory for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<InError, Destination> ObserverUpgradesToSelf for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
}

impl<InError, Destination> Observer for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.is_getting_activated = next;
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<InError, Destination> Tickable for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
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
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<InError, Destination> ObserverInput for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	type In = bool;
	type InError = InError;
}

impl<InError, Destination> ObservableOutput for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	InError: SignalBound,
{
	type Out = AdsrSignal;
	type OutError = InError;
}
