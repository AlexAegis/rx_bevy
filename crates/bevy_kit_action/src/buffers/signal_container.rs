use crate::Signal;

/// The most trivial signal buffer, holds a single value
#[derive(Debug, Default)]
pub struct SignalContainer<S: Signal> {
	pub signal: S,
	/// For change detection.
	///
	/// It's populated at the beginning of a frame, before this frames signals
	/// are read and propagated, so that the state of the whole
	/// [SignalContainer] is valid throughout the rest of the frame.
	pub last_frame_signal: S,
}
