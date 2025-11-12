use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, SignalBound, Subscriber, SubscriptionContext, Tick, Tickable};

use crate::{AdsrEnvelopePhase, AdsrEnvelopeState, AdsrSignal, operator::AdsrOperatorOptions};

// TODO: It'd be nice to control the envelope live, I guess that could be done by querying the subscriber itself, but it would be nicer to control the operator itself, in case there are many observers
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(RxSubscriber)]
#[rx_in(bool)]
#[rx_in_error(InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct AdsrSubscriber<InError, Destination>
where
	InError: SignalBound,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	#[destination]
	destination: Destination,
	is_getting_activated: bool,
	state: AdsrEnvelopeState,
	pub options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	InError: SignalBound,
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

impl<InError, Destination> Observer for AdsrSubscriber<InError, Destination>
where
	InError: SignalBound,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
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
	InError: SignalBound,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
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
		if self.options.reset_input_on_tick {
			self.is_getting_activated = false;
		}

		if !matches!(next.adsr_envelope_phase, AdsrEnvelopePhase::None) {
			self.destination.next(next, context);
		}
	}
}
