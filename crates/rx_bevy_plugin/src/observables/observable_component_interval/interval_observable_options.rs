use std::time::Duration;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObservableOptions {
	pub duration: Duration,
	/// Whether or not the first emission, `0` should happen on subscribe
	/// or after the duration had elapsed once.
	pub start_on_subscribe: bool,
}
