#[derive(Clone, Debug)]
pub enum QueueOverflowBehavior {
	/// Upon reaching the `max_queue_limit`, the oldest value in the queue will
	/// be dropped to make room for the new value
	DropOldest,
	/// Upon reaching the `max_queue_limit`, new emissions won't be accepted.
	IgnoreNext,
}

#[derive(Clone, Debug)]
pub struct ZipSubscriberOptions {
	/// To avoid one, rapidly emitting observable to grow the ZipSubscriber
	/// indefinitely, a max queue length can be set.
	/// Pushing new values will either be ignored, or drop the oldest one, to
	/// make room for the new value, depending on the `overflow_behavior`.
	/// Each individual observers internal queue will then be limited to this
	/// size.
	///
	/// By default, this is set to `100`
	pub max_queue_length: usize,

	/// By default, this is set to `DropOldest`
	pub overflow_behavior: QueueOverflowBehavior,
}

impl Default for ZipSubscriberOptions {
	fn default() -> Self {
		Self {
			max_queue_length: 100,
			overflow_behavior: QueueOverflowBehavior::DropOldest,
		}
	}
}
