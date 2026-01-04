use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use rx_core_traits::{SubscriptionWithTeardown, Teardown};

pub trait TrackTeardownExtension {
	fn tracked(prefix: &str) -> (Teardown, TeardownTracker);
}

impl TrackTeardownExtension for Teardown {
	fn tracked(prefix: &str) -> (Teardown, TeardownTracker) {
		let was_torn_down = Arc::new(AtomicBool::new(false));
		let was_torn_down_clone = was_torn_down.clone();
		(
			Teardown::new(move || {
				was_torn_down_clone.store(true, Ordering::Relaxed);
			}),
			TeardownTracker {
				was_torn_down,
				prefix: prefix.to_string(),
			},
		)
	}
}

#[derive(Clone)]
pub struct TeardownTracker {
	prefix: String,
	was_torn_down: Arc<AtomicBool>,
}

impl TeardownTracker {
	#[track_caller]
	pub fn assert_was_torn_down(&self) {
		assert!(
			self.was_torn_down.load(Ordering::Relaxed),
			"{} - Teardown did not run when it should have!",
			self.prefix
		)
	}

	#[track_caller]
	pub fn assert_yet_to_be_torn_down(&self) {
		assert!(
			!self.was_torn_down.load(Ordering::Relaxed),
			"{} - Teardown ran when it shouldn't have!",
			self.prefix
		)
	}
}

pub trait TrackedTeardownSubscriptionExtension {
	fn add_tracked_teardown(&mut self, prefix: &str) -> TeardownTracker;
}

impl<S> TrackedTeardownSubscriptionExtension for S
where
	S: SubscriptionWithTeardown,
{
	fn add_tracked_teardown(&mut self, prefix: &str) -> TeardownTracker {
		let (teardown, teardown_tracker) = Teardown::tracked(prefix);
		self.add_teardown(teardown);
		teardown_tracker
	}
}
