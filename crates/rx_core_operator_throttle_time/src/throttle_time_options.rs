use std::time::Duration;

/// The output behavior for a throttle window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThrottleOutputBehavior {
	/// Emit the value that starts the throttle window.
	LeadingOnly,
	/// Emit the most recent value observed when the throttle window ends.
	TrailingOnly,
	/// Emit both the first and last values observed in each throttle window.
	LeadingAndTrailing,
}

impl ThrottleOutputBehavior {
	#[inline]
	pub fn emits_leading(self) -> bool {
		matches!(self, Self::LeadingOnly | Self::LeadingAndTrailing)
	}

	#[inline]
	pub fn emits_trailing(self) -> bool {
		matches!(self, Self::TrailingOnly | Self::LeadingAndTrailing)
	}
}

/// Options for configuring the `throttle_time` operator.
#[derive(Clone, Copy, Debug)]
pub struct ThrottleTimeOptions {
	/// The throttle window duration.
	/// Default: `1s`.
	pub duration: Duration,
	/// Which emissions are produced in each throttle window.
	/// Default: `ThrottleOutput::LeadingAndTrailing`.
	pub output_behavior: ThrottleOutputBehavior,
}

impl ThrottleTimeOptions {
	/// Creates options with the given duration and default output.
	///
	/// Defaults:
	/// - `duration`: `1s`
	/// - `output`: `ThrottleOutput::LeadingAndTrailing`.
	pub fn new(duration: Duration) -> Self {
		Self {
			duration,
			output_behavior: ThrottleOutputBehavior::LeadingAndTrailing,
		}
	}

	#[must_use]
	pub fn with_output(mut self, output: ThrottleOutputBehavior) -> Self {
		self.output_behavior = output;
		self
	}
}

impl Default for ThrottleTimeOptions {
	/// Defaults:
	/// - `duration`: `1s`
	/// - `output`: `ThrottleOutput::LeadingAndTrailing`.
	fn default() -> Self {
		Self {
			duration: Duration::from_secs(1),
			output_behavior: ThrottleOutputBehavior::LeadingAndTrailing,
		}
	}
}
