use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin::default(),
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				unsubscribe.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn unsubscribe(mut example_entities: ResMut<MySubscriptions>) {
	example_entities.subscription.unsubscribe();
}

#[derive(Resource)]
struct MySubscriptions {
	subscription: SharedSubscription,
}

fn setup(mut commands: Commands, rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
	let subscription = KeyboardObservable::new(default(), rx_schedule_update_virtual.handle())
		.subscribe(PrintObserver::new("keyboard"));

	commands.insert_resource(MySubscriptions {
		subscription: SharedSubscription::new(subscription),
	});
}
