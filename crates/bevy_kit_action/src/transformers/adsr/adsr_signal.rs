use std::time::Duration;

use bevy::prelude::*;
use smallvec::SmallVec;

use crate::{Signal, SignalAggregator, SignalEvent};

use super::{AdsrEnvelopePhase, AdsrEnvelopePhaseTransition};

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub struct AdsrSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: AdsrEnvelopePhaseTransition,
	pub t: Duration,
	pub value: f32,
}

impl Signal for AdsrSignal {
	type Accumulator = AdsrSignalAccumulator;
	type Event = AdsrSignalEvent;
}

#[derive(Debug, Default, Reflect)]
pub struct AdsrSignalAccumulator;

impl SignalAggregator<AdsrSignal> for AdsrSignalAccumulator {
	fn combine(&self, _accumulator: AdsrSignal, next: AdsrSignal) -> AdsrSignal {
		next
	}
}

#[derive(Event, Debug)]
pub enum AdsrSignalEvent {
	Start,
	Fire,
	Sustain,
	Release,
	Stop,
	Restart,
}

impl SignalEvent<AdsrSignal> for AdsrSignalEvent {
	type SignalEventState = ();

	/// While we could calculate the phase transition here too, it is already done in the
	/// Transformer, as it's needed to know when the envelope ended
	fn from_signal_state(signal_state: &crate::SignalState<AdsrSignal>) -> SmallVec<[Self; 1]> {
		signal_state.signal.phase_transition.map_to_signal_events()
	}
}
