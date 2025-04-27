use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem, ActionSystemFor, Signal};

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
	S: Signal + 'static,
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

fn trigger_actions<A, S>(_commands: Commands, action_socket_query: Query<&mut ActionSocket<A>>)
where
	A: Action<Signal = S>,
	S: Signal + 'static,
{
	for action_socket in action_socket_query.iter() {
		// TODO: Add an ActionTriggerTarget component to be able to trigger other entities too, just like action source, if it's not present, then trigger self
		for (_action, _action_state) in action_socket.iter_signals() {
			// TODO: impl apply
			// TODO: FROM HERE !!!!! BufferedTransformerStage
			// TODO: FROM HERE !!!!! BufferedTransformerStage
			// TODO: FROM HERE !!!!! BufferedTransformerStage
			// TODO: FROM HERE !!!!!
			// TODO: FROM HERE !!!!!
			// TODO: FROM HERE !!!!!
			//	action_state.apply()
			// match action_state.phase {
			//     ActionEnvelopeState::Attack => ActionOnGoing { action },
			//     ActionEnvelopeState::Active => ActionOnGoing{ action },
			//     ActionEnvelopeState::Release => ActionEnd { action },
			// }

			// commands.trigger_targets(, target_entity);
		}
	}
}
