use bevy::{
	input::{ButtonState, InputSystem, keyboard::KeyboardInput},
	prelude::*,
	time::Stopwatch,
};

use crate::{ActionContext, ActionEnvelopeState, ActionKey, ActionState, ActionSystem};

pub struct KeyboardInputActionPlugin;

impl Plugin for KeyboardInputActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_keyboard_sink);
		app.add_systems(
			PreUpdate,
			keyboard_to_direct_input_action
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
	commands.spawn((KeyboardActionSink, ActionContext::<KeyCode>::default()));
}

fn keyboard_to_direct_input_action(
	mut keyboard_input_event_reader: EventReader<KeyboardInput>,
	mut keyboard_sink_query: Query<&mut ActionContext<KeyCode>, With<KeyboardActionSink>>,
	#[cfg(feature = "dev")] frame_count: Res<bevy::core::FrameCount>,
) {
	let Ok(mut keyboard_sink_action_context) = keyboard_sink_query.get_single_mut() else {
		error!("Keyboard sink isn't spawned!");
		return;
	};

	for keyboard_event in keyboard_input_event_reader.read() {
		if keyboard_event.repeat {
			// TODO: enable processing repeat events based on some config per key, should it be ignored, or should it re-trigger the action
			continue;
		}

		#[cfg(feature = "dev")]
		trace!("keyboard event {:?} {:?}", &keyboard_event, frame_count);

		let action_state = keyboard_sink_action_context
			.actions
			.entry(keyboard_event.key_code)
			.or_insert_with(|| ActionState::<KeyCode>::new(keyboard_event.key_code));

		match keyboard_event.state {
			ButtonState::Pressed => {
				action_state.elapsed = Stopwatch::new();
				action_state.phase = ActionEnvelopeState::Attack;
			}
			ButtonState::Released => {
				action_state.elapsed.pause();
				action_state.phase = ActionEnvelopeState::Release;
			}
		};
	}
}

#[derive(Default, Debug, Deref, Reflect)]
pub struct KeyboardInputActionData {
	active: bool,
}

impl ActionKey for KeyCode {
	type ActionData = KeyboardInputActionData;
}
