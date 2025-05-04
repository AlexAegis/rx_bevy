use std::time::Duration;

use bevy::prelude::*;

use crate::Signal;

use super::{
	AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, AdsrSignalAggregator, AdsrSignalEvent,
};

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, Default))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct AdsrSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: AdsrEnvelopePhaseTransition,
	pub t: Duration,
	pub value: f32,
}

impl Signal for AdsrSignal {
	type Aggregator = AdsrSignalAggregator;
	type Event = AdsrSignalEvent;
}
