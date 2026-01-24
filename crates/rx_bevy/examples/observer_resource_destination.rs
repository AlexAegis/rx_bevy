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
		.init_resource::<Counter>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

#[derive(Resource, Default, Debug)]
struct Counter(i32);

fn setup(rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
	let _s = JustObservable::new(1).subscribe(ResourceDestination::new(
		|mut counter: Mut<'_, Counter>, signal| {
			println!("Received signal: {:?}", signal);
			if let ObserverNotification::Next(value) = signal {
				counter.0 = value;
			}
			println!("Counter updated to: {:?}", counter);
		},
		rx_schedule_update_virtual.handle(),
	));
}
