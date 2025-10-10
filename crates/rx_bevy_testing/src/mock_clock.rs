use std::time::Duration;

use rx_bevy_core::Tick;

#[derive(Default)]
pub struct MockClock {
	now: Duration,
}

impl MockClock {
	pub fn elapse(&mut self, duration: Duration) -> Tick {
		self.now += duration;

		Tick {
			now: self.now,
			delta: duration,
		}
	}
}
