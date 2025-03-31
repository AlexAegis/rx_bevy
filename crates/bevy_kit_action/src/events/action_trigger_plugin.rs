use std::marker::PhantomData;

use bevy::{input::InputSystem, prelude::*};
use derive_where::derive_where;

use crate::{
	Action, ActionContext, ActionEnd, ActionEnvelopeState, ActionOnGoing, ActionStart,
	ActionSystem, ActionSystemFor,
};

/// Emit events
#[derive_where(Default)]
pub struct ActionTriggerPlugin<A, S = <A as Action>::Signal>
where
	A: Action<Signal = S>,
{
	_phantom_data_action: PhantomData<A>,
}

impl<A, S> Plugin for ActionTriggerPlugin<A, S>
where
	A: Action<Signal = S>,
	S: 'static,
{
	fn build(&self, app: &mut App) {
		// Clear actions before bevy would emit the current ones for this frame
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<A>::Trigger
				.after(ActionSystem::Mapped)
				.before(ActionSystem::Triggered),
		);

		app.add_systems(
			PreUpdate,
			trigger_actions::<A, S>.in_set(ActionSystemFor::<A>::Trigger),
		);
	}
}

fn trigger_actions<A, S>(
	mut commands: Commands,
	action_context_query: Query<(Entity, &ActionContext<A>)>,
) where
	A: Action<Signal = S>,
	S: 'static,
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
