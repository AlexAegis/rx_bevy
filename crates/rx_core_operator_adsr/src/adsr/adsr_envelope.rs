use std::time::Duration;

use crate::AdsrEnvelopePhase;

use bevy_math::{
	Curve,
	curve::{EaseFunction, EasingCurve},
};

// TODO: Maybe this could actually be a DAHDSR (delay, attack, hold, decay, sustain, release) envelope. (But keep the name Adsr, it's more known)
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
	/// TODO: If there is any behavior regarding values outside of this range, mention it here
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
}

/*
/// TODO: Impl, this is for transforming adsr envelopes back to booleans, this will likely be a second operator_adsr_actuator
#[derive(Debug, Clone, Copy, Default)]
pub struct ActionActuationPreferences {
	trigger_rule: ActionTriggerRule,
	release_rule: ActionReleaseRule,
}
*/

/// Describe at what stage the source action must be at for this action to
/// also be activated.
/// This also serves as an activation condition as an action may not always
/// be at the stage required to open the gate of this action.
///
/// By `default` these rules map the source's activation directly.
///
/// Synonyms: `Rising Edge` | `Gate On` | `Trigger`
#[derive(Debug, Clone, Copy, Default)]
pub enum ActionTriggerRule {
	/// Immediately when the source actions gate is opened, this action also
	/// starts getting activated without having to wait until the source action's
	/// `attackTime` has passed and "fired".
	///
	/// Synonyms: `Direct`
	#[default]
	Immediate,
	/// Start activating this action when the source action has fired
	OnFire,
	/// Only start activating this action when the source action has fully
	/// decayed and is only sustaining itself.
	/// If the source action wasn't activated long enough to fully decay, this
	/// action won't be triggered with this rule.
	OnDecay,
	/// Start activating this action when the previous actions ADSR value has
	/// reached this Threshold.
	///
	/// For example if you take a typical linear ADSR envelope that attacks
	/// from 0.0 to 1.0. Setting this to 0.0 is equivalent of
	/// [ActionGateOpenRule::Immediate] and setting it to 1.0 is equivalent of
	/// [ActionGateOpenRule::OnFire]
	/// TODO: Maybe for joysticks this would act as the deadzone! But for that it has to be a Vec2/3 that would require this to have a dimensionality
	Threshold(f32),
}

/// Describes at what stage the source action must be at for this action to
/// deactivate.
///
/// By `default` these rules map the source's activation directly.
/// Synonyms: `Falling Edge` | `Gate Off`
#[derive(Debug, Clone, Copy, Default)]
pub enum ActionReleaseRule {
	/// Stop this action from getting activated when the source has also
	/// stopped getting activated
	/// Synonyms: `Direct`
	#[default]
	OnRelease,
	/// Keep activating this action until it has reached
	OnEnd,
	/// Stop activating this action on the next frame it was activated
	OneShot,
	/// Stop activating this action when the previous actions ADSR value has
	/// dipped below this Threshold.
	Threshold(f32),
}

/// Unlike a true ADSR envelope, which is monotonically increasing during the
/// attack phase and monotonically decreasing afterwards, custom envelope
/// functions could violate this property (e.g., a BounceIn-BounceOut function).
pub struct ThresholdActivationRule {
	/// The Threshold that has to be crossed by the envelope
	pub threshold: f32,
	/// Depending on the ADSR envelopes easing function, a threshold could be
	/// crossed multiple times, this option ensures only the first one
	/// triggers an activation, per input activation
	pub once: bool,
}

impl Default for ThresholdActivationRule {
	fn default() -> Self {
		Self {
			once: true,
			threshold: 0.0,
		}
	}
}
