use bevy::prelude::*;
use std::time::Duration;

use super::AdsrEnvelopePhase;

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct AdsrEnvelope {
	pub attack_time: Duration,
	/// How does the attack duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	pub attack_fn: Option<fn(f32) -> f32>,
	pub decay_time: Duration,
	/// How does the decay duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	pub decay_fn: Option<fn(f32) -> f32>,
	/// What value should be reached by decay. Should be between 0.0 and 1.0,
	/// TODO: If there is any behavior regarding values outside of this range, mention it here
	pub sustain_volume: f32,

	/// How long after release the action still be alive
	pub release_time: Duration,
	/// How does the release duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	pub release_fn: Option<fn(f32) -> f32>,
}

const LINEAR: fn(f32) -> f32 = |v: f32| v;

impl AdsrEnvelope {
	pub fn evaluate(&self, t: Duration, active: bool) -> (AdsrEnvelopePhase, f32) {
		let phase = self.determine_current_phase(t, active);

		let envelope_fn = match phase {
			AdsrEnvelopePhase::Attack => self.attack_fn.unwrap_or(LINEAR),
			AdsrEnvelopePhase::Decay => self.decay_fn.unwrap_or(LINEAR),
			AdsrEnvelopePhase::Release => self.release_fn.unwrap_or(LINEAR),
			_ => LINEAR,
		};

		// TODO: Finish signal strength calculation

		(phase, 0.0)
	}

	fn determine_current_phase(&self, t: Duration, active: bool) -> AdsrEnvelopePhase {
		if active {
			if t < self.attack_time {
				AdsrEnvelopePhase::Attack
			} else if t < self.attack_time + self.decay_time {
				AdsrEnvelopePhase::Decay
			} else {
				AdsrEnvelopePhase::Sustain
			}
		} else if self.attack_time + self.decay_time < t {
			AdsrEnvelopePhase::Release
		} else {
			AdsrEnvelopePhase::None
		}
	}
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct ActionActuationPreferences {
	trigger_rule: ActionTriggerRule,
	release_rule: ActionReleaseRule,
}

/// Describe at what stage the source action must be at for this action to
/// also be activated.
/// This also serves as an activation condition as an action may not always
/// be at the stage required to open the gate of this action.
///
/// By `default` these rules map the source's activation directly.
///
/// Synonyms: `Rising Edge` | `Gate On` | `Trigger`
#[derive(Debug, Clone, Copy, Default, Reflect)]
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
	/// reached this treshold.
	///
	/// For example if you take a typical linear ADSR envelope that attacks
	/// from 0.0 to 1.0. Setting this to 0.0 is equivalent of
	/// [ActionGateOpenRule::Immediate] and setting it to 1.0 is equivalent of
	/// [ActionGateOpenRule::OnFire]
	/// TODO: Maybe for joysticks this would act as the deadzone! But for that it has to be a Vec2/3 that would require this to have a dimensionality
	Treshold(f32),
}

/// Describes at what stage the source action must be at for this action to
/// deactivate.
///
/// By `default` these rules map the source's activation directly.
/// Synonyms: `Falling Edge` | `Gate Off`
#[derive(Debug, Clone, Copy, Default, Reflect)]
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
	/// dipped below this treshold.
	Treshold(f32),
}

/// Unlike a true ADSR envelope, which is monotonically increasing during the
/// attack phase and monotonically decreasing afterwards, custom envelope
/// functions could violate this property (e.g., a BounceIn-BounceOut function).
pub struct TresholdActivationRule {
	/// The treshold that has to be crossed by the envelope
	treshold: f32,
	/// Depending on the ADSR envelope, a treshold can be cros
	once: bool,
}

impl Default for TresholdActivationRule {
	fn default() -> Self {
		Self {
			once: true,
			treshold: 0.0,
		}
	}
}
