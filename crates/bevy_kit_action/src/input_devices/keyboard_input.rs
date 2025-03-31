use bevy::{
	input::{ButtonState, InputSystem, keyboard::KeyboardInput},
	prelude::*,
	time::Stopwatch,
};

use crate::{
	Action, ActionContext, ActionEnvelopeState, ActionState, ActionSystem, AdsrSocket,
	BooleanSocket, Signal, SignalDimension,
};

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
	commands.spawn((
		KeyboardActionSink,
		ActionContext::<KeyboardPressAction>::default(),
	));
}

fn keyboard_to_direct_input_action(
	mut keyboard_input_event_reader: EventReader<KeyboardInput>,
	mut keyboard_sink_query: Query<
		&mut ActionContext<KeyboardPressAction>,
		With<KeyboardActionSink>,
	>,
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
			.or_insert_with(|| ActionState::<KeyboardPressAction>::new(keyboard_event.key_code));

		// A direct activation of the action
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

impl Signal for KeyCode {
	const DIMENSION: SignalDimension = SignalDimension::ONE;
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct KeyboardPressAction;

impl Action for KeyboardPressAction {
	type Signal = KeyCode;
	type InputSocket = BooleanSocket;
	type OutputSocket = BooleanSocket;
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct KeyboardAdsrAction;

impl Action for KeyboardAdsrAction {
	type Signal = KeyCode;
	type InputSocket = BooleanSocket;
	type OutputSocket = AdsrSocket;
	// TODO: Expand on this idea, is this the same thing as Signal or not? data is not needed for now so lets just rename or reuse that
	// type Socket = ActionSocketBoolean;
}
