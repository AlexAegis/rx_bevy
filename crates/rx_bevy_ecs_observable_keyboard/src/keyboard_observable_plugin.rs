use std::marker::PhantomData;

use bevy_app::{App, Plugin};
use bevy_ecs::{event::EventReader, schedule::ScheduleLabel, system::Query};
use bevy_input::keyboard::KeyboardInput;
use rx_bevy_plugin::Subscription;

use crate::KeyboardSubscription;

#[derive(Default)]
pub struct KeyboardObservablePlugin<S>
where
	S: ScheduleLabel,
{
	_phantom_data: PhantomData<S>,
}

impl<S> Plugin for KeyboardObservablePlugin<S>
where
	S: ScheduleLabel + Default,
{
	fn build(&self, app: &mut App) {
		app.add_systems(S::default(), keyboard_subscriber_system);
	}
}

pub(crate) fn keyboard_subscriber_system(
	mut keyboard_input_events: EventReader<KeyboardInput>,
	mut keyboard_subscriber_query: Query<&mut Subscription<KeyboardSubscription>>,
) {
	for keyboard_input in keyboard_input_events.read() {
		for mut keyboard_subscriber in keyboard_subscriber_query.iter_mut() {
			keyboard_subscriber.write(keyboard_input.clone());
		}
	}
}
