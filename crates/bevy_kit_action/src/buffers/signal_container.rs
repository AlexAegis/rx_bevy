use crate::Signal;

/// The most trivial signal buffer, holds a single value
#[derive(Debug, Default)]
pub struct SignalContainer<S: Signal> {
	pub signal: S,
}
