use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem, ActionSystemFor, Signal, SignalEvent};

use super::ActionEvent;

/// Emit events defined by the signals of each action
#[derive_where(Default)]
pub struct ActionTriggerPlugin<A>
where
	A: Action,
{
	_phantom_data_action: PhantomData<A>,
}

impl<A> Plugin for ActionTriggerPlugin<A>
where
	A: Action,
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
			trigger_actions::<A>.in_set(ActionSystemFor::<A>::Trigger),
		);
	}
}

fn trigger_actions<A>(
	mut commands: Commands,
	mut action_socket_query: Query<(Entity, &mut ActionSocket<A>)>,
) where
	A: Action,
{
	for (entity, mut action_socket) in action_socket_query.iter_mut() {
		for (action, signal_state) in action_socket.iter_containers_mut() {
			for event in <<A::Signal as Signal>::Event as SignalEvent<A::Signal>>::from_signal_state(
				signal_state,
			) {
				commands.trigger_targets(
					ActionEvent {
						action: *action,
						signal: signal_state.signal,
						event,
					},
					entity,
				);
			}
		}
	}
}
