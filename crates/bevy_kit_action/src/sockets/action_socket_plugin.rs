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
		app.add_systems(
			PreUpdate,
			set_last_frame_data::<A>.in_set(ActionSystem::Reset),
		);
	}
}

fn set_last_frame_data<A: Action>(mut action_socket_query: Query<&mut ActionSocket<A>>) {
	for mut action_socket in action_socket_query.iter_mut() {
		for (_, signal_container) in action_socket.iter_mut() {
			signal_container.last_frame_signal = std::mem::take(&mut signal_container.signal);
		}
	}
}
