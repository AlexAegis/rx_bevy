use bevy::prelude::*;

/// ? Probably will be generic on Scheduler
pub struct RxPlugin {}

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, tick);
	}
}

fn tick() {
	println!("tick");
}
