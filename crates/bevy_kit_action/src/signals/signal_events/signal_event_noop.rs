use bevy::prelude::*;
use smallvec::SmallVec;

use crate::{Signal, SignalEvent, SignalEventVec, SignalState};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Event, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct SignalNoopEvent;

impl<S: Signal> SignalEvent<S> for SignalNoopEvent {
	type SignalEventState = ();

	fn from_signal_state(_signal_state: &SignalState<S>) -> SignalEventVec<Self> {
		SmallVec::new()
	}
}
