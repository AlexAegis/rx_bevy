use std::marker::PhantomData;

use bevy::{input::InputSystem, prelude::*};
use derive_where::derive_where;

use crate::{
	ActionContext, ActionEnd, ActionEnvelopeState, ActionKey, ActionOnGoing, ActionStart,
	ActionSystem, ActionSystemFor,
};

/// Emit events
#[derive_where(Default)]
pub struct ActionTriggerPlugin<Action, Data = <Action as ActionKey>::ActionData>
where
	Action: ActionKey<ActionData = Data>,
{
	_phantom_data_action: PhantomData<Action>,
}

impl<Action, Data> Plugin for ActionTriggerPlugin<Action, Data>
where
	Action: ActionKey<ActionData = Data>,
	Data: 'static,
{
	fn build(&self, app: &mut App) {
		// Clear actions before bevy would emit the current ones for this frame
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<Action>::Trigger
				.after(ActionSystem::Mapped)
				.before(ActionSystem::Triggered),
		);

		app.add_systems(
			PreUpdate,
			trigger_actions::<Action, Data>.in_set(ActionSystemFor::<Action>::Trigger),
		);
	}
}

fn trigger_actions<Action, Data>(
	mut commands: Commands,
	action_context_query: Query<(Entity, &ActionContext<Action>)>,
) where
	Action: ActionKey<ActionData = Data>,
	Data: 'static,
{
	for (target_entity, action_context) in action_context_query.iter() {
		// TODO: Add an ActionTriggerTarget component to be able to trigger other entities too, just like action source, if it's not present, then trigger self
		for (action, action_state) in action_context.actions.iter() {
			// match action_state.phase {
			//     ActionEnvelopeState::Attack => ActionOnGoing { action },
			//     ActionEnvelopeState::Active => ActionOnGoing{ action },
			//     ActionEnvelopeState::Release => ActionEnd { action },
			// }

			// commands.trigger_targets(, target_entity);
		}
	}
}
