use bevy::prelude::*;

use crate::KeyboardInputActionSocketPlugin;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(KeyboardInputActionSocketPlugin);
	}
}
