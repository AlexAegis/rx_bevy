use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSystem, ConnectorTerminal};

use super::{ActionSocket, SocketConnections};

#[derive_where(Default)]
pub struct ActionSocketPlugin<A: Action> {
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> Plugin for ActionSocketPlugin<A> {
	fn build(&self, app: &mut App) {
		app.register_type::<ActionSocket<A>>();
		// Maybe not here
		app.register_type::<SocketConnections<A>>();

		#[cfg(feature = "debug_ui")]
		app.add_plugins(crate::ActionSignalDebugUiPlugin::<A>::default());

		app.add_systems(
			PreUpdate,
			(reset_sockets::<A>, reset_terminals::<A>)
				.in_set(ActionSystem::Reset)
				.before(ActionSystem::InputSocketWrite),
		);
	}
}

/// Sets last frames signal and resets the current signal to it's [Default] if not latching
/// Also resets the write flag
fn reset_sockets<A: Action>(mut action_socket_query: Query<&mut ActionSocket<A>>) {
	for mut action_socket in action_socket_query.iter_mut() {
		let is_latching = action_socket.latching;
		for (_, signal_state) in action_socket.iter_mut() {
			if is_latching {
				signal_state.last_frame_signal = signal_state.signal;
			} else {
				signal_state.last_frame_signal = std::mem::take(&mut signal_state.signal);
			}
			signal_state.written = false;
		}
	}
}

fn reset_terminals<A: Action>(mut terminal_query: Query<&mut ConnectorTerminal<A>>) {
	for mut terminal in terminal_query.iter_mut() {
		for (_, signal_accumulator) in terminal.iter_mut() {
			signal_accumulator.signal = <A as Action>::Signal::default();
			signal_accumulator.written = false;
			signal_accumulator.all_other_writes_this_frame.clear();
		}
	}
}
