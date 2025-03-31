use bevy::{prelude::*, utils::HashMap};

use crate::{Action, KeyboardInputActionPlugin};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(KeyboardInputActionPlugin);
	}
}
