use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_time::Virtual;

use crate::RxScheduler;

/// A collection of default plugins
/// TODO: Add a dyn vec of schedules and a chainable .schedule_on method, and the default version adds Update
pub struct RxPlugin;

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			RxScheduler::<Update, Virtual>::default(),
			RxScheduler::<PostUpdate, Virtual>::default(),
		));
	}
}
