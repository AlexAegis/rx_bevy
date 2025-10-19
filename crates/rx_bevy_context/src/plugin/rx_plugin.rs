use bevy_app::{App, Plugin, PostUpdate, Update};

use bevy_time::Virtual;

use crate::RxScheduler;

/// A collection of default plugins
pub struct RxPlugin;

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		#[cfg(feature = "debug")]
		app.add_plugins(crate::DebugInspectorPlugin);

		app.add_plugins((
			RxScheduler::<Update, Virtual>::default(),
			RxScheduler::<PostUpdate, Virtual>::default(),
		));
	}
}
