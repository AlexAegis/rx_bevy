use std::time::Duration;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct IntervalObservableOptions {
	pub duration: Duration,
	/// Whether or not the first emission, `0` should happen on subscribe
	/// or after the duration had elapsed once.
	pub start_on_subscribe: bool,
	/// If the internal timer rolls over multiple times during a single tick,
	/// all of them will result in an emissin. To prevent emitting too much
	/// during a particularly large tick, for example during a lagged frame,
	/// this limit ensures at most this many emissions can happen during a
	/// single tick.
	///
	/// It doesn't need to be a `usize` as the number it's compared against is
	/// a `u32` coming from [bevy_time::Timer::times_finished_this_tick]
	pub max_emissions_per_tick: u32,
}
