use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem, ActionSystemFor, Signal, SignalEvent};

use super::{ActionEvent, SocketTriggerTarget};

/// Emit events defined by the signals of each action
#[derive_where(Default)]
pub struct ActionEventTriggerPlugin<A>
where
	A: Action,
{
	_phantom_data_action: PhantomData<A>,
}

impl<A> Plugin for ActionEventTriggerPlugin<A>
where
	A: Action,
{
	fn build(&self, app: &mut App) {
		#[cfg(feature = "reflect")]
		app.register_type::<SocketTriggerTarget>();
		// Clear actions before bevy would emit the current ones for this frame
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<A>::Trigger
				.after(ActionSystem::PropagationDone)
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
	mut action_socket_query: Query<(Entity, &mut ActionSocket<A>, Option<&SocketTriggerTarget>)>,
) where
	A: Action,
{
	for (entity, mut action_socket, action_trigger_target) in action_socket_query.iter_mut() {
		let target_entity = match action_trigger_target.cloned().unwrap_or_default() {
			SocketTriggerTarget::This => entity,
			SocketTriggerTarget::Other(other_entity) => other_entity,
		};

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
					target_entity,
				);
			}
		}
	}
}
