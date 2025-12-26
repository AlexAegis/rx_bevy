use std::time::Duration;

#[derive(Debug, Clone)]
pub struct IntervalObservableOptions {
	/// How much time must elapse between emissions
	///
	/// Default: 1 sec
	pub duration: Duration,
	/// Whether or not the first emission, `0` should
	/// happen on subscribe or after the duration had elapsed once.
	/// Note that when this is `true`, the first emission happens "on subscribe"
	/// meaning, immediately, and outside of the scheduler!
	///
	/// Default: false
	pub start_on_subscribe: bool,
	/// If the internal timer rolls over multiple times during a single tick,
	/// all of them will result in an emission. To prevent emitting too much
	/// during a particularly large tick, for example during a lagged frame,
	/// this limit ensures at most this many emissions can happen.
	///
	/// This setting is mostly relevant for short intervals, shorter than
	/// a slow framerate. (Ignore the stable, high framerate you're targeting,
	/// what matters is what happens when it slows down!)
	///
	/// - When downstream is expensive and it's okay to lose some emissions:
	///   You may want to set this to 1 if you don't want to make a lag-spike
	///   even worse, when reacting to the interval rolling over multiple times
	///   would perform something expensive multiple times.
	/// - When time-keeping is important and downstream computation is cheap:
	///   You can safely keep this at a higher value to not lose any time
	///   measured.
	///
	/// Since setting this to `0` would result in the interval not doing
	/// anything, 1 will be used instead.
	///
	/// Default: 10
	pub max_emissions_per_tick: usize,
}

impl Default for IntervalObservableOptions {
	fn default() -> Self {
		Self {
			duration: Duration::from_millis(1000),
			max_emissions_per_tick: 10,
			start_on_subscribe: false,
		}
	}
}
