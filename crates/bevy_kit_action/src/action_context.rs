use std::{
	cell::{Cell, OnceCell, RefCell},
	marker::PhantomData,
	sync::Arc,
	time::Duration,
};

use bevy::{prelude::*, utils::HashMap};

use derive_where::derive_where;

use crate::{ActionKey, ActionState};

/// Where Actions arrive (Sink?)
/// TODO: It would be so much better to not have these hashmaps here
#[derive(Component, Clone, Debug, Reflect)]
#[derive_where(Default)]
pub struct ActionContext<A: ActionKey> {
	_phantom_data_action_key: PhantomData<A>,
	/// TODO(benchmark): Consider that in any given frame maybe only 1 or 2 actions are happening, maybe a simple Vec would be faster!
	pub actions: HashMap<A, ActionState<A>>,
	/// Used to determine action phase transitions
	pub(crate) last_frame_actions: HashMap<A, ActionState<A>>,
	activation_preferences: HashMap<A, ActionActivationPreferences>,
	envelope_preferences: HashMap<A, ActionEnvelopePreferences>,
	/// Where actions are triggered from. Can point to a gamepad entity,
	/// or the special Keyboard entity, or another entity with an actionContext
	/// as long as there is mapping defined between this context and that
	/// contexts action type, mapping and triggering will occur.
	pub sources: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct ActionEnvelopePreferences {
	attack_time: Duration,
	/// How does the attack duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	attack_fn: Option<fn(f32) -> f32>,
	decay_time: Duration,
	/// How does the decay duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	decay_fn: Option<fn(f32) -> f32>,
	/// What value should be reached by decay. Should be between 0.0 and 1.0,
	/// TODO: If there is any behavior regarding values outside of this range, mention it here
	sustain_volume: f32,
	/// How long after release the action still be alive
	release_time: Duration,
	/// How does the release duration shape the envelope
	/// Input range between 0.0 and 1.0
	/// Default: Linear mapping
	#[reflect(ignore)]
	release_fn: Option<fn(f32) -> f32>,
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct ActionActivationPreferences {
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

// TODO: Maybe not needed to have a `once`, normal ADSR envelope only rise on
// attack, and only fall after that. So unlike a regular function, there's no
// risk of  crossing a treshold more than once. But, if we allow that a mapping
// also have an easing function, then it's possible (for example with bounce)
// Then this becomes necessary
// pub struct TresholdActivationRule {
// 	/// The treshold that has to be crossed by the envelope
// 	treshold: f32,
// 	/// Depending on the ADSR envelope, a treshold can be cros
// 	once: bool,
//  /// TODO: Maybe certain actions could explicitly ignore any smoothed values and read the raw envelope
//  from_unmapped: bool,
// }
//
// impl Default for TresholdActivationRule {
// 	fn default() -> Self {
// 		Self {
// 			once: true,
// 			treshold: 0.0,
//			from_unmapped: false,
// 		}
// 	}
// }
