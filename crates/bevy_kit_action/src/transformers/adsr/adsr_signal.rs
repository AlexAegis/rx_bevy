use std::time::Duration;

use bevy::prelude::*;

use crate::{Signal, SignalAggregator};

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
}

#[derive(Debug, Default, Reflect)]
pub struct AdsrSignalAccumulator;

impl SignalAggregator<AdsrSignal> for AdsrSignalAccumulator {
	fn combine(&self, _accumulator: AdsrSignal, next: AdsrSignal) -> AdsrSignal {
		next
	}
}
