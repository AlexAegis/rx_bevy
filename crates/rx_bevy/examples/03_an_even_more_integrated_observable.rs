use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

/// Example 03 - An even more integrated observable
///
/// This next example now stores the subscription in an entity, so I no longer
/// need to store it myself.
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
		.add_systems(Startup, setup_subscription)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

/// Now it sends events to a destination entity which simply observes the
/// signals using an actual Bevy Observer!
fn setup_subscription(
	mut commands: Commands,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
	let destination_entity = commands
		.spawn_empty()
		.observe(|signal: Trigger<RxSignal<usize>>| println!("{:?}", signal.signal()))
		.id();

	// You should still put either this `EntitySubscription`, or the entity
	// within (obtained using `entity_subscription.entity()`) somewhere so
	// you can cancel this by despawning it!
	let _entity_subscription = IntervalObservable::new(
		IntervalObservableOptions {
			duration: Duration::from_secs(1),
			start_on_subscribe: true,
			max_emissions_per_tick: 1,
		},
		rx_schedule_update_virtual.handle(),
	)
	.with_commands(commands, rx_schedule_update_virtual.handle())
	.subscribe(EntityDestination::new(
		destination_entity,
		rx_schedule_update_virtual.handle(),
	));
}
