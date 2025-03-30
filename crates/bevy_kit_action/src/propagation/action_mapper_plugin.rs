use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	ActionContext, ActionEnvelopePhaseTransition, ActionEnvelopeState, ActionKey, ActionSource,
	ActionState, ActionSystem, ActionSystemFor,
};

use super::ActionMap;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive_where(Default)]
pub struct ActionMapPlugin<
	FromAction,
	ToAction,
	FromData = <FromAction as ActionKey>::ActionData,
	ToData = <ToAction as ActionKey>::ActionData,
> where
	FromAction: ActionKey<ActionData = FromData>,
	ToAction: ActionKey<ActionData = ToData>,
	ToData: From<FromAction::ActionData>,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}

impl<FromAction, ToAction, FromData, ToData> Plugin
	for ActionMapPlugin<FromAction, ToAction, FromData, ToData>
where
	FromAction: ActionKey<ActionData = FromData>,
	ToAction: ActionKey<ActionData = ToData>,
	FromData: 'static,
	ToData: 'static + From<FromAction::ActionData>,
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
			map_actions::<FromAction, ToAction, FromData, ToData>
				.in_set(ActionSystemFor::<ToAction>::Map),
		);
	}
}

fn map_actions<FromAction, ToAction, FromData, ToData>(
	mut to_action_context_query: Query<(
		&mut ActionContext<ToAction>,
		&ActionMap<FromAction, ToAction, FromData, ToData>,
		&ActionSource<FromAction>,
	)>,
	from_action_context_query: Query<&ActionContext<FromAction>>,
) where
	FromAction: ActionKey<ActionData = FromData>,
	ToAction: ActionKey<ActionData = ToData>,
	FromData: 'static,
	ToData: 'static + From<FromAction::ActionData>,
{
	for (mut to_action_context, action_map, action_source) in to_action_context_query.iter_mut() {
		// TODO: If FromAction is keyboard, automatically use that.
		for from_action_context in from_action_context_query.iter_many(action_source.sources.iter())
		{
			let to_actions: Vec<ToAction> = to_action_context.actions.keys().copied().collect();
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
	ToAction: ActionKey,
{
	fn apply<FromAction>(
		&mut self,
		other: &ActionState<FromAction>,
		previous: Option<&ActionState<ToAction>>,
	) where
		FromAction: ActionKey,
	{
		// TODO: If into is implemented, otherwise use a mapper function, OR forget this
		// self.action = other.action.into();

		// TODO: Not sure if this makes sense, maybe based on some action kind? Like ActionKind::Instant, Hold or whatever
		//self.elapsed = other.elapsed.clone();
		//self.phase = other.phase.clone();
		self.active = other.active; // TODO: Only activate the to action, if a treshhold has been reached.
	}
}

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
