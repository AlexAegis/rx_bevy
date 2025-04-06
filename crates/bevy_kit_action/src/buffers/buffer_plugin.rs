use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem};

// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: START HERE
// TODO: So right now every action needs its own plugins, and maybe thats okay, but what if data and transform and buffer configs would be separate, maybe its a bad idea
#[derive_where(Default)]
pub struct BufferPlugin<A: Action> {
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> Plugin for BufferPlugin<A> {
	fn build(&self, app: &mut App) {
		app.add_systems(PreUpdate, push_buffers::<A>.in_set(ActionSystem::Reset));
	}
}

fn push_buffers<A: Action>(mut a_query: Query<&mut ActionSocket<A>>) {
	for mut action_socket in a_query.iter_mut() {
		for (action, signal_container) in action_socket.state.iter() {
			// action_socket.buffer
		}
	}
}
