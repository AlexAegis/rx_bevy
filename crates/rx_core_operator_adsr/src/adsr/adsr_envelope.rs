use std::time::Duration;

use crate::{AdsrEnvelopeChange, AdsrEnvelopePhase};

use bevy_math::{
	Curve,
	curve::{EaseFunction, EasingCurve},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct AdsrEnvelope {
	pub attack_time: Duration,
	/// How does the attack duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub attack_easing: Option<EaseFunction>,
	pub decay_time: Duration,
	/// How does the decay duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub decay_easing: Option<EaseFunction>,
	/// What value should be reached by decay. Should be between 0.0 and 1.0,
	pub sustain_volume: f32,

	/// How long after release the action still be alive
	pub release_time: Duration,
	/// How does the release duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	pub release_easing: Option<EaseFunction>,
}

impl AdsrEnvelope {
	pub fn evaluate(
		&self,
		is_getting_activated: bool,
		t: Duration,
		deactivation_value: Option<f32>,
		deactivation_time: Option<Duration>,
	) -> (f32, AdsrEnvelopePhase) {
		let (phase, start_time, end_time) =
			self.determine_current_phase_and_start_time(is_getting_activated, t, deactivation_time);

		let sustain = self.sustain_volume.clamp(0.0, 1.0);

		let value = match phase {
			AdsrEnvelopePhase::Attack => {
				let curve =
					EasingCurve::new(0.0, 1.0, self.attack_easing.unwrap_or(EaseFunction::Linear));
				let pos = ((t - start_time).as_secs_f32()
					/ (end_time - start_time).as_secs_f32().max(f32::EPSILON))
				.clamp(0.0, 1.0);
				curve.sample_clamped(pos)
			}
			AdsrEnvelopePhase::Decay => {
				let curve = EasingCurve::new(
					1.0,
					sustain,
					self.decay_easing.unwrap_or(EaseFunction::Linear),
				);
				let pos = ((t - start_time).as_secs_f32()
					/ (end_time - start_time).as_secs_f32().max(f32::EPSILON))
				.clamp(0.0, 1.0);
				curve.sample_clamped(pos)
			}
			AdsrEnvelopePhase::Sustain => sustain,
			AdsrEnvelopePhase::Release => {
				let curve = EasingCurve::new(
					deactivation_value.unwrap_or(sustain),
					0.0,
					self.release_easing.unwrap_or(EaseFunction::Linear),
				);
				let pos = ((t - start_time).as_secs_f32()
					/ (end_time - start_time).as_secs_f32().max(f32::EPSILON))
				.clamp(0.0, 1.0);
				curve.sample_clamped(pos)
			}
			_ => 0.0,
		};

		(value.clamp(0.0, 1.0), phase)
	}

	fn determine_current_phase_and_start_time(
		&self,
		is_getting_activated: bool,
		t: Duration,
		deactivation_time: Option<Duration>,
	) -> (AdsrEnvelopePhase, Duration, Duration) {
		match (is_getting_activated, deactivation_time) {
			(false, None) => (
				AdsrEnvelopePhase::None,
				Duration::from_millis(0),
				Duration::from_millis(0),
			),
			(true, None) => {
				if t < self.attack_time {
					(
						AdsrEnvelopePhase::Attack,
						Duration::from_millis(0),
						self.attack_time,
					)
				} else if t < self.attack_time + self.decay_time {
					(
						AdsrEnvelopePhase::Decay,
						self.attack_time,
						self.attack_time + self.decay_time,
					)
				} else {
					(
						AdsrEnvelopePhase::Sustain,
						self.attack_time + self.decay_time,
						self.attack_time + self.decay_time, // This should really be the current time, but since the sustain level is a fixed value, there's nothing to interpolate
					)
				}
			}
			(_, Some(deactivation_time)) => {
				if t < (deactivation_time + self.release_time) {
					(
						AdsrEnvelopePhase::Release,
						deactivation_time,
						deactivation_time + self.release_time,
					)
				} else {
					(
						AdsrEnvelopePhase::None,
						Duration::from_millis(0),
						Duration::from_millis(0),
					)
				}
			}
		}
	}

	pub fn apply_change(&mut self, change: AdsrEnvelopeChange) {
		if let Some(attack_time) = change.attack_time {
			self.attack_time = attack_time;
		};

		if let Some(attack_easing) = change.attack_easing {
			self.attack_easing = Some(attack_easing);
		};

		if let Some(decay_time) = change.decay_time {
			self.decay_time = decay_time;
		};

		if let Some(decay_easing) = change.decay_easing {
			self.decay_easing = Some(decay_easing);
		};

		if let Some(sustain_volume) = change.sustain_volume {
			self.sustain_volume = sustain_volume;
		};

		if let Some(release_time) = change.release_time {
			self.release_time = release_time;
		};

		if let Some(release_easing) = change.release_easing {
			self.release_easing = Some(release_easing);
		};
	}
}

#[cfg(test)]
mod tests {
	use std::time::Duration;

	use crate::{AdsrEnvelope, AdsrEnvelopeChange};

	#[test]
	fn adsr_envelope_apply_change_overrides_config() {
		let mut envelope = AdsrEnvelope::default();

		envelope.apply_change(AdsrEnvelopeChange {
			attack_time: Some(Duration::from_millis(10)),
			attack_easing: None,
			decay_time: Some(Duration::from_millis(20)),
			decay_easing: None,
			sustain_volume: Some(0.75),
			release_time: Some(Duration::from_millis(30)),
			release_easing: None,
		});

		assert_eq!(envelope.attack_time, Duration::from_millis(10));
		assert_eq!(envelope.decay_time, Duration::from_millis(20));
		assert_eq!(envelope.release_time, Duration::from_millis(30));
		assert_eq!(envelope.sustain_volume, 0.75);
	}
}
