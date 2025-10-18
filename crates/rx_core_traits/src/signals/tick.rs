use std::time::Duration;

/// Used for scheduling, subscriptions are ticked with this event
#[derive(Debug, Clone)]
pub struct Tick {
	pub now: Duration,
	pub delta: Duration,
}
