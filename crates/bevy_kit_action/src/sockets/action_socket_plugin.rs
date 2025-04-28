use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSystem};

use super::ActionSocket;

#[derive_where(Default)]
pub struct ActionSocketPlugin<A: Action> {
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> Plugin for ActionSocketPlugin<A> {
	fn build(&self, app: &mut App) {
		app.register_type::<ActionSocket<A>>();

		#[cfg(feature = "debug_ui")]
		app.add_plugins(crate::ActionSignalDebugUiPlugin::<A>::default());

		app.add_systems(
			PreUpdate,
			set_last_frame_data::<A>
				.in_set(ActionSystem::Reset)
				.before(ActionSystem::InputSocketWrite),
		);
	}
}

fn set_last_frame_data<A: Action>(mut action_socket_query: Query<&mut ActionSocket<A>>) {
	for mut action_socket in action_socket_query.iter_mut() {
		let is_latching = action_socket.latching;
		for (_, signal_container) in action_socket.iter_mut() {
			if is_latching {
				signal_container.last_frame_signal = signal_container.signal;
			} else {
				signal_container.last_frame_signal = std::mem::take(&mut signal_container.signal);
			}
		}
	}
}
