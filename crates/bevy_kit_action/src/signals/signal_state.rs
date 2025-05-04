use bevy::{prelude::*, reflect::Reflect};

use crate::Signal;

use super::SignalEvent;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone, Debug))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct SignalState<S: Signal> {
	#[deref]
	#[cfg_attr(feature = "serialize", serde(bound(deserialize = "S: Signal")))]
	pub signal: S,

	/// For change detection.
	///
	/// It's populated at the beginning of a frame, before this frames signals
	/// are read and propagated, so that the state of the whole
	/// [SignalContainer] is valid throughout the rest of the frame.
	#[cfg_attr(feature = "serialize", serde(bound(deserialize = "S: Signal")))]
	pub last_frame_signal: S,

	/// When events need some persistent state to be fired, like tracking
	/// when it was last fired. This field is NOT reset between frames.
	#[cfg_attr(feature = "serialize", serde(bound(deserialize = "S: Signal")))]
	pub(crate) event_state: <<S as Signal>::Event as SignalEvent<S>>::SignalEventState,
}
