use std::time::Duration;

use bevy::prelude::*;
use smallvec::SmallVec;

use crate::{SignalEvent, SignalEventState, SignalEventVec, SignalState};

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

#[derive(Event, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub enum SignalEventBool {
	/// Fired on the signals rising edge, when it just turned from `false` to `true`.
	Activated,
	/// Fired on the signals falling edge, when it just turned from `true` to `false`.
	Deactivated,
	/// Continuous event, fired each frame the signal is true
	Active,
}

impl SignalEvent<bool> for SignalEventBool {
	type SignalEventState = SignalBooleanEventState;

	fn from_signal_state(signal_state: &SignalState<bool>) -> SignalEventVec<Self> {
		let mut events = SmallVec::<[Self; 2]>::new();
		if !signal_state.last_frame_signal && signal_state.signal {
			events.push(Self::Activated);
		} else if signal_state.last_frame_signal && !signal_state.signal {
			events.push(Self::Deactivated);
		}

		if signal_state.signal {
			events.push(Self::Active);
		}

		events
	}
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct SignalBooleanEventState {
	last_activation: Option<Duration>,
}

impl SignalEventState for SignalBooleanEventState {}
