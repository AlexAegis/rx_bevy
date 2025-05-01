use bevy::reflect::Reflect;

use crate::Signal;

use super::SignalEvent;

#[derive(Debug, Default, Reflect)]
pub struct SignalState<S: Signal> {
	pub signal: S,
	/// For change detection.
	///
	/// It's populated at the beginning of a frame, before this frames signals
	/// are read and propagated, so that the state of the whole
	/// [SignalContainer] is valid throughout the rest of the frame.
	pub last_frame_signal: S,

	// When events need some persistent state to be fired, like tracking
	// when it was last fired. This field is NOT reset between frames.
	pub(crate) event_state: <<S as Signal>::Event as SignalEvent<S>>::SignalEventState,

	/// Tracks if this signal was written this frame or not. This is used to
	/// determine if the value should be simple set, or according to the
	/// accumulation behavior.
	///
	/// Resets on Reset
	pub(crate) written: bool,
}

/// Holds all the Signals written this frame, then at the Aggregation stage
/// it will be combined to a single signal that can be emitted
/// TODO: Actually implement whats above
#[derive(Debug, Default, Reflect)]
pub struct SignalAccumulator<S: Signal> {
	/// Reverts to its [Default] on [ActionSystem::Reset][`crate::ActionSystem::Reset`]
	/// for non-latching sockets.
	pub signal: S,

	/// Tracks if this signal was written this frame or not. This is used to
	/// determine if the value should be simple set, or according to the
	/// accumulation behavior.
	///
	/// Reverts to `false` on [ActionSystem::Reset][`crate::ActionSystem::Reset`]
	pub(crate) written: bool,

	/// In most use-cases accumulation isn't used, as long as only one write
	/// happens this frame, this Vec will remain empty
	///
	/// Empties on [ActionSystem::Reset][`crate::ActionSystem::Reset`]
	pub(crate) all_other_writes_this_frame: Vec<S>,
}
