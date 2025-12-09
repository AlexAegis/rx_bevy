use std::{
	ops::{Add, AddAssign, Sub},
	time::Duration,
};

/// Used for scheduling, subscriptions are ticked with this event
// TODO: Move this to the ticking scheduler once the tick channel is removed
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick {
	pub elapsed_since_start: Duration,
	pub delta: Duration,
}

impl Tick {
	pub fn new(elapsed_since_start: Duration, delta: Duration) -> Self {
		Self {
			elapsed_since_start,
			delta,
		}
	}

	pub fn update(&mut self, tick: Tick) {
		if self.elapsed_since_start < tick.elapsed_since_start {
			self.elapsed_since_start = tick.elapsed_since_start;
		}
	}

	pub fn is_newer_than(&self, other: Option<&Tick>) -> bool {
		match other {
			Some(other_tick) => self.elapsed_since_start > other_tick.elapsed_since_start,
			None => true,
		}
	}
}

impl Add<Duration> for Tick {
	type Output = Tick;

	fn add(self, rhs: Duration) -> Self::Output {
		Tick {
			elapsed_since_start: self.elapsed_since_start + rhs,
			delta: rhs,
		}
	}
}

impl Sub<Duration> for Tick {
	type Output = Tick;

	fn sub(self, rhs: Duration) -> Self::Output {
		Tick {
			elapsed_since_start: self.elapsed_since_start - rhs,
			delta: rhs,
		}
	}
}

impl AddAssign<Duration> for Tick {
	fn add_assign(&mut self, rhs: Duration) {
		self.elapsed_since_start += rhs;
		self.delta = rhs;
	}
}
