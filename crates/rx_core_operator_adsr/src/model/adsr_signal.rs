use std::time::Duration;

use super::{AdsrEnvelopePhase, AdsrEnvelopePhaseTransition};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "serialize", feature = "reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::ReflectDefault;

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(
	feature = "reflect",
	derive(bevy_reflect::Reflect),
	reflect(Debug, Clone, Default)
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct AdsrSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: AdsrEnvelopePhaseTransition,
	pub t: Duration,
	pub value: f32,
}
