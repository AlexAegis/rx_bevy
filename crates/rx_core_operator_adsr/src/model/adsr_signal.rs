use std::time::Duration;

use super::{AdsrEnvelopePhase, AdsrEnvelopePhaseTransition};

#[derive(Debug, Copy, Clone, Default)]
pub struct AdsrSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: AdsrEnvelopePhaseTransition,
	pub t: Duration,
	pub value: f32,
}
