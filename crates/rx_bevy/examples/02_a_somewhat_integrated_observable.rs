use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

/// Example 02 - A somewhat integrated observable
///
/// This next example now actually interacts with the ECS by sending `RxSignal`
/// events to another entity!
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.init_resource::<MySubscriptions>()
		.add_systems(Startup, setup_subscription)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

/// I still need to store the subscription myself
#[derive(Resource, Default, Deref, DerefMut)]
struct MySubscriptions(SharedSubscription);

/// Now send events to a destination entity by simply observing the signals
/// using an actualy Bevy Observer!
fn setup_subscription(
	mut commands: Commands,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
	mut my_subscriptions: ResMut<MySubscriptions>,
) {
	let destination_entity = commands
		.spawn_empty()
		.observe(|signal: Trigger<RxSignal<usize>>| println!("{:?}", signal.signal()))
		.id();

	let subscription = IntervalObservable::new(
		IntervalObservableOptions {
			duration: Duration::from_secs(1),
			start_on_subscribe: true,
			max_emissions_per_tick: 1,
		},
		rx_schedule_update_virtual.handle(),
	)
	.subscribe(EntityDestination::new(
		destination_entity,
		rx_schedule_update_virtual.handle(),
	));

	my_subscriptions.add(subscription);
}
