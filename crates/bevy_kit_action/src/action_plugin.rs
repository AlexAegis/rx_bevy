use bevy::prelude::*;

use crate::KeyboardInputActionPlugin;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(KeyboardInputActionPlugin);
	}
}
