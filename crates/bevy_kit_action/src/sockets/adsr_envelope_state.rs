use bevy::prelude::*;

/// ActionPhase mimics an ADSR envelope, where actions don't necessarily Fire
/// the moment they start, maybe it needs to be held for a time to do that.
/// After firing the action may still getting activated, as long as this happens
/// the phase is sustained and once it stops it enters the release phase
/// TODO: Figure out what this value should control! For now it could be stored somewhere as a 1.0 that is then available as a voltage or something field on the actionState
#[derive(Clone, Default, Debug, Reflect)]
pub enum ActionEnvelopeState {
	/// The default state, nothing is happening here
	#[default]
	None,
	/// Starts on frame 0.
	///
	/// TODO: Add an attackEasing configuration that defaults to linear
	///
	/// TODO: Figure out how `attackTime` should be configured
	/// For example: actions that do not need to be held this has a 0 frame
	/// length and immediately advances to the next phase
	Attack,
	/// Starts after `attackTime` and lasts until `decayTime`
	/// The moment the action enters this phase, the action is considered **Fired**.
	/// May stay here for decayTime which by default is 0
	/// TODO: Add a decayEasing config that defaults to linear
	/// TODO: figure out where and configure decayTime
	Decay,
	/// Starts after `attackTime` and `decayTime` and lasts until the action is no longer activated
	Sustain,
	/// Starts after the action stopped being activated and lasts until `releaseTime`
	/// TODO: figure out where to configure `releaseTime`
	/// TODO: Add a releaseEasing config that defaults to linear
	Release,
}

/// Describes what happened between this and the last frame, aside from None
/// other transitions are only present for a single frame, and can be used
/// in the same fashion as `just_pressed`
#[derive(Clone, Debug, Default, Reflect)]
pub enum AdsrEnvelopePhaseTransition {
	/// Used between any other transition, mostly when no transition happens
	#[default]
	None,
	/// [ActionEnvelopeState::None] -> [ActionEnvelopeState::Attack]
	/// The action has started getting activated.
	/// ! Only present if the action has an attackTime other than 0, otherwise
	/// ! will only observe the Fire transition
	/// TODO: Maybe I could add some special hidden transitions, that will fire both observables so Start and Fire can both be reliable listened to, especially if attackTime is modifiable
	Start,
	/// [ActionEnvelopeState::Attack] -> [ActionEnvelopeState::Decay]
	/// The action is now fully activated and `attackTime` has passed.
	/// If there was no `attackTime` this is the first phase transition that
	/// occurs, otherwise it's [ActionEnvelopePhaseTransition::Start]
	Fire,
	/// [ActionEnvelopeState::Decay] -> [ActionEnvelopeState::Sustain]
	///
	Sustain,
	/// [ActionEnvelopeState::Sustain] -> [ActionEnvelopeState::Release]
	/// The action is no longer activated
	Release,
	/// [ActionEnvelopeState::Release] -> [ActionEnvelopeState::None]
	End,
}

/// TODO: Maybe this whole envelop thing could be a condition or at least an optional things called actuation. then ADSR wouldn't be a prominent thing after all, just a feature. But then actions would need sockets? as subtypes and matching sockets could only be mapped together, or if one implements a Trait to convert. After all, the input really is just a boolean, lasting for a time (plus gamepad stuff and mouse, envelopes should be on top of them, optionally)
fn determine_phase_transition(
	previous_frame: &ActionEnvelopeState,
	current_frame: &ActionEnvelopeState,
) -> AdsrEnvelopePhaseTransition {
	match (previous_frame, current_frame) {
		(ActionEnvelopeState::None, ActionEnvelopeState::Attack) => {
			AdsrEnvelopePhaseTransition::Start
		}
		(ActionEnvelopeState::None, ActionEnvelopeState::Decay) => {
			// When there is no attackTime
			AdsrEnvelopePhaseTransition::Fire
		}

		(ActionEnvelopeState::Attack, ActionEnvelopeState::Decay) => {
			AdsrEnvelopePhaseTransition::Fire
		}
		(ActionEnvelopeState::Decay, ActionEnvelopeState::Sustain) => {
			AdsrEnvelopePhaseTransition::Sustain
		}
		(ActionEnvelopeState::Sustain, ActionEnvelopeState::Release) => {
			AdsrEnvelopePhaseTransition::Release
		}
		(ActionEnvelopeState::Release, ActionEnvelopeState::None) => {
			AdsrEnvelopePhaseTransition::End
		}
		_ => AdsrEnvelopePhaseTransition::None,
	}
}
