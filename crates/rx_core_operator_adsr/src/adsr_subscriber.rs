use core::marker::PhantomData;
use std::time::Duration;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

use crate::{
	AdsrEnvelopePhase, AdsrEnvelopeState, AdsrSignal, AdsrTrigger, operator::AdsrOperatorOptions,
};

// TODO: It'd be nice to control the envelope live, I guess that could be done by querying the subscriber itself, but it would be nicer to control the operator itself, in case there are many observers
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(RxSubscriber)]
#[rx_in(AdsrTrigger)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct AdsrSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	#[destination]
	destination: Destination,
	is_getting_activated: bool,
	last_signal_was_none: bool,
	state: AdsrEnvelopeState,
	pub options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	pub fn new(destination: Destination, options: AdsrOperatorOptions) -> Self {
		Self {
			destination,
			options,
			is_getting_activated: false,
			last_signal_was_none: false,
			state: AdsrEnvelopeState::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, Destination> Observer for AdsrSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.is_getting_activated = next.activated;

		if let Some(envelope_change) = next.envelope_changes {
			self.options.envelope.apply_change(envelope_change);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	/// TODO: MIGRATE IT INTO THE SCHEDULER
	#[inline]
	fn tick(&mut self, elapsed_since_start: Duration, tick_delta: Duration) {
		let next = self.state.calculate_output(
			self.options.envelope,
			self.is_getting_activated,
			elapsed_since_start,
			tick_delta,
		);
		if self.options.reset_input_on_tick {
			self.is_getting_activated = false;
		}

		let current_phase_is_none = matches!(next.adsr_envelope_phase, AdsrEnvelopePhase::None);

		// If `always_emit_none`, it always emits.
		// If the current phase isn't `None`, then it also should emit because it's a useful value.
		// If the last signal was not `None`, then the current value should be emitted even if it's
		// a `None` to have at least one `None` emitted at the end of an activation.
		if self.options.always_emit_none || !current_phase_is_none || !self.last_signal_was_none {
			self.destination.next(next);
		}

		self.last_signal_was_none = current_phase_is_none;
	}
}
