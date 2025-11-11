use std::time::Duration;

use rx_core_traits::Tick;

#[derive(Default)]
pub struct MockClock {
	index: usize,
	now: Duration,
}

impl MockClock {
	pub fn elapse(&mut self, duration: Duration) -> Tick {
		self.now += duration;

		let index = self.index;
		self.index += 1;

		Tick {
			index,
			now: self.now,
			delta: duration,
		}
	}
}
