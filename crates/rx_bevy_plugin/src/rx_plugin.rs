use bevy::prelude::*;

use crate::RxScheduler;

/// A collection of default plugins
pub struct RxPlugin;

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((RxScheduler::on(Update), RxScheduler::on(PostUpdate)));
	}
}
