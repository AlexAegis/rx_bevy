use bevy::prelude::*;

use crate::{SignalEvent, SignalEventVec};

use super::{AdsrEnvelopePhase, AdsrSignal};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Event, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub enum AdsrSignalEvent {
	/// Fired immediately upon activation
	Start,
	/// Fired when the attack duration had elapsed and the signal reached its peak
	Fire,
	/// Fired when the signal had decayed but it's still getting activated
	Sustain,
	/// Fired when the signal enters the release phase after sustain, or
	/// decay if sustain wasn't reached, or attack if decay wasn't reached
	Release,
	/// Fired when the signal finished fully, or restarted
	Stop,
	/// Fired when the signal was re-triggered before [Release][`AdsrSignalEvent::Release`] could've finished
	Restart,
	/// Fired continuously from attack to the end of release every frame
	Active,
}

impl SignalEvent<AdsrSignal> for AdsrSignalEvent {
	type SignalEventState = ();

	/// While we could calculate the phase transition here too, it is already done in the
	/// Transformer, as it's needed to know when the envelope ended
	fn from_signal_state(signal_state: &crate::SignalState<AdsrSignal>) -> SignalEventVec<Self> {
		let mut events = signal_state.signal.phase_transition.map_to_signal_events();

		if signal_state.signal.adsr_envelope_phase != AdsrEnvelopePhase::None {
			events.push(Self::Active);
		}

		events
	}
}
