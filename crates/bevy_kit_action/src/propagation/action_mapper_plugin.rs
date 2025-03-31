use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionContext, ActionEnvelopePhaseTransition, ActionEnvelopeState, ActionSource,
	ActionState, ActionSystem, ActionSystemFor, Signal,
};

use super::ActionMap;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive_where(Default)]
pub struct ActionMapPlugin<
	FromAction,
	ToAction,
	FromData = <FromAction as Action>::Signal,
	ToData = <ToAction as Action>::Signal,
> where
	FromAction: Action<Signal = FromData>,
	ToAction: Action<Signal = ToData>,
	ToData: From<FromAction::Signal>,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}

impl<FromAction, ToAction, FromSignal, ToSignal> Plugin
	for ActionMapPlugin<FromAction, ToAction, FromSignal, ToSignal>
where
	FromAction: Action<Signal = FromSignal>,
	ToAction: Action<Signal = ToSignal>,
	FromSignal: Signal + 'static,
	ToSignal: Signal + 'static + From<FromAction::Signal>,
{
	fn build(&self, app: &mut App) {
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Map
				.after(ActionSystemFor::<FromAction>::Map)
				.after(ActionSystem::Input)
				.before(ActionSystem::Mapped),
		);

		// Actions are triggered backwards compared to mapping
		// TODO: Does it matter? Which is better? This is kinda like bubbling. Should it be a crate feature?
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			map_actions::<FromAction, ToAction, FromSignal, ToSignal>
				.in_set(ActionSystemFor::<ToAction>::Map),
		);
	}
}

fn map_actions<FromAction, ToAction, FromSignal, ToSignal>(
	mut to_action_context_query: Query<(
		&mut ActionContext<ToAction>,
		&ActionMap<FromAction, ToAction, FromSignal, ToSignal>,
		&ActionSource<FromAction>,
	)>,
	from_action_context_query: Query<&ActionContext<FromAction>>,
) where
	FromAction: Action<Signal = FromSignal>,
	ToAction: Action<Signal = ToSignal>,
	FromSignal: Signal + 'static,
	ToSignal: Signal + 'static + From<FromAction::Signal>,
{
	for (mut to_action_context, action_map, action_source) in to_action_context_query.iter_mut() {
		// TODO: If FromAction is keyboard, automatically use that.
		for from_action_context in from_action_context_query.iter_many(action_source.sources.iter())
		{
			let a = to_action_context.actions.keys().map(|a| a);
			let to_actions: Vec<ToSignal> = to_action_context
				.actions
				.keys()
				.map(|a| a)
				.copied()
				.collect();
			let last_frame_actions = to_action_context.last_frame_actions.clone(); // Clone to avoid borrowing issues

			for to_action in to_actions {
				if let Some(to_action_state) = to_action_context.actions.get_mut(&to_action) {
					if let Some(mapped_from_action) = action_map.action_map.get(&to_action) {
						if let Some(from_action_state) =
							from_action_context.actions.get(mapped_from_action)
						{
							// At this stage during mapping, only activation is forwarded
							to_action_state
								.apply(from_action_state, last_frame_actions.get(&to_action));
						}
					}
				}
			}
		}
	}
}

impl<ToAction> ActionState<ToAction>
where
	ToAction: Action,
{
	fn apply<FromAction>(
		&mut self,
		other: &ActionState<FromAction>,
		previous: Option<&ActionState<ToAction>>,
	) where
		FromAction: Action,
	{
		// TODO: If into is implemented, otherwise use a mapper function, OR forget this
		// self.action = other.action.into();

		// TODO: Not sure if this makes sense, maybe based on some action kind? Like ActionKind::Instant, Hold or whatever
		//self.elapsed = other.elapsed.clone();
		//self.phase = other.phase.clone();
		self.active = other.active; // TODO: Only activate the to action, if a treshold has been reached.
	}
}

/// TODO: Maybe this whole envelop thing could be a condition or at least an optional things called actuation. then ADSR wouldn't be a prominent thing after all, just a feature. But then actions would need sockets? as subtypes and matching sockets could only be mapped together, or if one implements a Trait to convert. After all, the input really is just a boolean, lasting for a time (plus gamepad stuff and mouse, envelopes should be on top of them, optionally)
fn determine_phase_transition(
	previous_frame: &ActionEnvelopeState,
	current_frame: &ActionEnvelopeState,
) -> ActionEnvelopePhaseTransition {
	match (previous_frame, current_frame) {
		(ActionEnvelopeState::None, ActionEnvelopeState::Attack) => {
			ActionEnvelopePhaseTransition::Start
		}
		(ActionEnvelopeState::None, ActionEnvelopeState::Decay) => {
			// When there is no attackTime
			ActionEnvelopePhaseTransition::Fire
		}

		(ActionEnvelopeState::Attack, ActionEnvelopeState::Decay) => {
			ActionEnvelopePhaseTransition::Fire
		}
		(ActionEnvelopeState::Decay, ActionEnvelopeState::Sustain) => {
			ActionEnvelopePhaseTransition::Sustain
		}
		(ActionEnvelopeState::Sustain, ActionEnvelopeState::Release) => {
			ActionEnvelopePhaseTransition::Release
		}
		(ActionEnvelopeState::Release, ActionEnvelopeState::None) => {
			ActionEnvelopePhaseTransition::End
		}
		_ => ActionEnvelopePhaseTransition::None,
	}
}
