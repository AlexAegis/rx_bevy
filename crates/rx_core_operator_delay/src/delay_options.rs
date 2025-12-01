use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct DelayOperatorOptions {
	/// How much to delay each upstream emission before re-emitted downstream?
	///
	/// Default: 1 second
	pub delay: Duration,
}

impl Default for DelayOperatorOptions {
	fn default() -> Self {
		Self {
			delay: Duration::from_secs(1),
		}
	}
}
