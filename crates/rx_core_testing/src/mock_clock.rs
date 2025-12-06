use std::time::Duration;

use rx_core_traits::Tick;

#[derive(Default)]
pub struct MockClock {
	elapsed_since_start: Duration,
}

impl MockClock {
	pub fn elapse(&mut self, duration: Duration) -> Tick {
		self.elapsed_since_start += duration;

		//let index = self.index;
		//self.index += 1;

		Tick {
			elapsed_since_start: self.elapsed_since_start,
			delta: duration,
		}
	}
}
