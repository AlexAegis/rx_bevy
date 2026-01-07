use std::{
	ops::{Add, AddAssign, Deref},
	time::Duration,
};

use rx_core_common::WorkTick;

/// Used for scheduling, subscriptions are ticked with this event
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick {
	now: Duration,
}

impl Tick {
	pub fn new(now: Duration) -> Self {
		Self { now }
	}

	pub fn update(&mut self, tick: Tick) {
		if self.now < tick.now {
			self.now = tick.now;
		}
	}
}

impl WorkTick for Tick {
	#[inline]
	fn now(&self) -> Duration {
		self.now
	}
}

impl Deref for Tick {
	type Target = Duration;

	fn deref(&self) -> &Self::Target {
		&self.now
	}
}

impl Add<Duration> for Tick {
	type Output = Tick;

	fn add(self, rhs: Duration) -> Self::Output {
		Tick {
			now: self.now + rhs,
		}
	}
}

impl AddAssign<Duration> for Tick {
	fn add_assign(&mut self, rhs: Duration) {
		self.now += rhs;
	}
}

#[cfg(test)]
mod test {
	use std::{ops::Deref, time::Duration};

	use crate::Tick;

	#[test]
	fn should_deref_to_duration() {
		let tick = Tick::new(Duration::from_millis(1000));
		assert!(!tick.deref().is_zero());
	}

	#[test]
	fn should_be_able_to_increment_using_add_assign() {
		let mut tick = Tick::new(Duration::from_millis(1000));
		tick += Duration::from_millis(500);
		assert_eq!(*tick, Duration::from_millis(1500));
	}

	#[test]
	fn should_be_able_to_add_a_duration_to_a_tick() {
		let tick = Tick::new(Duration::from_millis(1000)) + Duration::from_millis(1200);
		assert_eq!(*tick, Duration::from_millis(2200));
	}

	mod update {

		use super::*;

		#[test]
		fn should_be_able_to_update_a_tick_with_a_newer_tick() {
			let mut tick = Tick::new(Duration::from_millis(1000));
			tick.update(Tick::new(Duration::from_millis(2000)));
			assert_eq!(*tick, Duration::from_millis(2000));
		}

		#[test]
		fn should_not_be_able_to_update_a_tick_with_an_older_tick() {
			let mut tick = Tick::new(Duration::from_millis(1000));
			tick.update(Tick::new(Duration::from_millis(20)));
			assert_eq!(*tick, Duration::from_millis(1000));
		}
	}
}
