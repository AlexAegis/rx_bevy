use std::time::Duration;

/// Used for scheduling, subscriptions are ticked with this event
#[derive(Debug, Clone)]
pub struct Tick {
	/// The index of this tick, used to discard repeated ticks in contexts where
	/// multiple sources are expected to tick one thing, such as shared
	/// subscribers like the `RcSubscriber`.
	///
	/// > Even on a 32 bit target, running at a 144hz tickrate, it will still
	/// > take almost a year to overflow.
	/// >
	/// > On 64 bit targets it would take 4059319764 years to overflow
	/// > 18446744073709551615 / (144 * 60 * 60 * 24 * 365.25)
	pub index: usize,
	pub now: Duration,
	pub delta: Duration,
}

impl Tick {
	pub fn is_newer_than(&self, other: Option<&Tick>) -> bool {
		match other {
			Some(other_tick) => self.index > other_tick.index,
			None => true,
		}
	}
}
