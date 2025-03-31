use std::marker::PhantomData;

use bevy::{
	input::{
		ButtonState, InputSystem,
		keyboard::{Key, KeyboardInput},
	},
	prelude::*,
	time::Stopwatch,
	utils::HashSet,
};

use crate::{
	Action, ActionContext, ActionEnvelopeState, ActionSocket, ActionState, ActionSystem,
	AdsrSocket, InputSocket, OutputSocket, Signal, SignalDimension,
};

pub struct KeyboardInputActionPlugin;

impl Plugin for KeyboardInputActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_keyboard_sink);
		app.add_systems(
			PreUpdate,
			forward_keyboard_to_socket
				.run_if(on_event::<KeyboardInput>)
				.in_set(ActionSystem::Input)
				.after(InputSystem),
		);
	}
}

#[derive(Component, Default, Clone, Debug, Reflect)]
#[require(Name(|| Name::new("KeyboardActionSink")))]
pub struct KeyboardActionSink;

fn setup_keyboard_sink(mut commands: Commands) {
	commands.spawn((KeyboardActionSink, KeyboardInputSocket::default()));
}

fn forward_keyboard_to_socket(
	mut keyboard_input_event_reader: EventReader<KeyboardInput>,
	mut keyboard_socket_query: Query<(
		&mut KeyboardInputSocket,
		Option<&KeyboardInputSocketOptions>,
	)>,
	#[cfg(feature = "dev")] frame_count: Res<bevy::core::FrameCount>,
) {
	for keyboard_event in keyboard_input_event_reader.read() {
		#[cfg(feature = "dev")]
		trace!("keyboard event {:?} {:?}", &keyboard_event, frame_count);

		let value = match keyboard_event.state {
			ButtonState::Pressed => true,
			ButtonState::Released => false,
		};

		for (mut keyboard_socket, keyboard_socket_options) in keyboard_socket_query.iter_mut() {
			if !keyboard_socket_options
				.map(|p| p.allow_repeat)
				.unwrap_or_default()
				&& keyboard_event.repeat
			{
				continue;
			}

			keyboard_socket.write(&keyboard_event.key_code, &value);
		}
	}
}

#[derive(Component, Default, Debug)]
pub struct KeyboardInputSocketOptions {
	allow_repeat: bool,
}

impl Signal for KeyCode {
	const DIMENSION: SignalDimension = SignalDimension::ZERO;
}

impl Action for KeyCode {
	type Signal = Self;
}

pub type KeyboardInputSocket = ActionSocket<KeyCode, bool>;
/*
#[derive(Component, Debug)]
struct KeyCodeSocket<K> {
	is_pressed: bool,
	_phantom_data_key: PhantomData<K>,
}

impl<K> OutputSocket<K> for KeyCodeSocket<K> {
	type Data = bool;
	fn read(&self) -> Self::Data {
		self.is_pressed
	}
}

/// TODO: Maybe this implementation shouldn't exist
impl<K> InputSocket<K> for KeyCodeSocket<K> {
	type Data = bool;

	fn write(&mut self, value: &Self::Data) {
		self.is_pressed = *value;
	}
}
*/
