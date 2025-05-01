use bevy::prelude::*;

use crate::{Action, ActionSocketPlugin, ActionTriggerPlugin, KeyboardInputActionSocketPlugin};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(KeyboardInputActionSocketPlugin);
	}
}

pub trait ActionApp {
	fn register_action<A: Action>(&mut self) -> &mut Self;
}

impl ActionApp for App {
	fn register_action<A: Action>(&mut self) -> &mut Self {
		#[cfg(feature = "reflect")]
		self.register_type::<A>();

		self.add_plugins(ActionSocketPlugin::<A>::default());
		self.add_plugins(ActionTriggerPlugin::<A>::default());

		self
	}
}
