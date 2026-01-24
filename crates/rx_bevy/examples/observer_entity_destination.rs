use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use examples_common::send_message;
use rx_bevy::prelude::*;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

fn setup(rx_schedule_update_virtual: RxSchedule<Update, Virtual>, mut commands: Commands) {
	let destination_entity = commands
		.spawn_empty()
		.observe(|signal: On<RxSignal<i32>>| println!("Received value: {:?}", signal.event()))
		.id();

	let _s = JustObservable::new(1).subscribe(EntityDestination::new(
		destination_entity,
		rx_schedule_update_virtual.handle(),
	));

	// If you have `observable_fn` feature enabled
	let _s = just(2).subscribe(EntityDestination::new(
		destination_entity,
		rx_schedule_update_virtual.handle(),
	));
}
