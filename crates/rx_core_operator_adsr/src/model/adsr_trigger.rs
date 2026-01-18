use std::time::Duration;

use bevy_math::curve::EaseFunction;

#[derive(Debug, Copy, Clone, Default)]
pub struct AdsrTrigger {
	pub activated: bool,
	pub envelope_changes: Option<AdsrEnvelopeChange>,
}

impl From<bool> for AdsrTrigger {
	fn from(activated: bool) -> Self {
		Self {
			activated,
			envelope_changes: None,
		}
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AdsrEnvelopeChange {
	pub attack_time: Option<Duration>,
	/// How does the attack duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub attack_easing: Option<EaseFunction>,
	pub decay_time: Option<Duration>,
	/// How does the decay duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub decay_easing: Option<EaseFunction>,
	/// What value should be reached by decay. Should be between 0.0 and 1.0,
	pub sustain_volume: Option<f32>,

	/// How long after release the action still be alive
	pub release_time: Option<Duration>,
	/// How does the release duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub release_easing: Option<EaseFunction>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_true() {
		let trigger_true: AdsrTrigger = true.into();
		assert!(trigger_true.activated);
		assert!(trigger_true.envelope_changes.is_none());
	}

	#[test]
	fn from_false() {
		let trigger_false: AdsrTrigger = false.into();
		assert!(!trigger_false.activated);
		assert!(trigger_false.envelope_changes.is_none());
	}
}
