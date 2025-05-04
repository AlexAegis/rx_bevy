use bevy::{
	input::{ButtonState, InputSystem, keyboard::KeyboardInput},
	log::trace,
	prelude::*,
};

use crate::{Action, ActionApp, ActionSocket, ActionSystem, SignalWriter};

use super::KeyboardActionSink;

pub struct KeyboardInputActionSocketPlugin;

impl Plugin for KeyboardInputActionSocketPlugin {
	fn build(&self, app: &mut App) {
		app.register_action::<KeyCode>();

		app.add_systems(Startup, setup_keyboard_sink);
		app.add_systems(
			PreUpdate,
			forward_keyboard_to_socket
				.run_if(on_event::<KeyboardInput>)
				.in_set(ActionSystem::InputSocketWrite)
				.after(InputSystem),
		);
	}
}

fn setup_keyboard_sink(mut commands: Commands) {
	commands.spawn((KeyboardActionSink, KeyboardInputSocket::new_latching()));
}

fn forward_keyboard_to_socket(
	mut keyboard_input_event_reader: EventReader<KeyboardInput>,
	mut keyboard_socket_query: Query<(
		&mut KeyboardInputSocket,
		Option<&KeyboardInputSocketOptions>,
	)>,
	#[cfg(feature = "dev")] frame_count: Res<bevy::diagnostic::FrameCount>,
) {
	for keyboard_event in keyboard_input_event_reader.read() {
		#[cfg(feature = "dev")]
		trace!("keyboard event {:?} {:?}", &keyboard_event, frame_count);

		let value = match keyboard_event.state {
			ButtonState::Pressed => true,
			ButtonState::Released => false,
		};

		for (mut keyboard_socket, keyboard_socket_options) in keyboard_socket_query.iter_mut() {
			if keyboard_event.repeat
				&& !keyboard_socket_options
					.map(|p| p.allow_repeat)
					.unwrap_or_default()
			{
				continue;
			}

			keyboard_socket.write(&keyboard_event.key_code, value);
		}
	}
}

#[derive(Component, Default, Debug)]
pub struct KeyboardInputSocketOptions {
	allow_repeat: bool,
}

impl Action for KeyCode {
	type Signal = bool;
}

pub type KeyboardInputSocket = ActionSocket<KeyCode>;
