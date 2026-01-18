use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

/// Mistake Example 10 - The mistake of subscribing every frame
///
/// It's easy to accidentally subscribe to an observable every frame if you
/// issue the subscribe command within an `Update` system!
///
/// Remember that an `Observable` is kinda like a factory function. Not only
/// this would re-create it every frame, but spawn a new **independent**
/// subscription from it every frame!
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
		.add_systems(Update, setup_subscription)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

fn setup_subscription(
	mut commands: Commands,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
	let destination_entity = commands
		.spawn_empty()
		.observe(|signal: Trigger<RxSignal<usize>>| println!("{:?}", signal.signal()))
		.id();

	let observable_entity = commands
		.spawn(
			IntervalObservable::new(
				IntervalObservableOptions {
					duration: Duration::from_secs(1),
					start_on_subscribe: true,
					max_emissions_per_tick: 1,
				},
				rx_schedule_update_virtual.handle(),
			)
			// .map(|i| i.to_string()) // This would change the output type of the observable, making the subscribe command below fail!
			.into_component(),
		)
		.id();

	// This is now **not** an `EntitySubscription`, as the subscription
	// will be made once the command executes! It's just an `Entity`!
	// Put it somewhere so you can despawn it!
	let _subscription_entity = commands.subscribe(
		observable_entity,
		EntityDestination::<usize, Never>::new(
			destination_entity,
			rx_schedule_update_virtual.handle(),
		),
	);
}
