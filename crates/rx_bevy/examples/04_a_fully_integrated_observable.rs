use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

/// Example 04 - A fully integrated observable
///
/// This next example now stores the observable itself in an entity!
///
/// > Note that these "levels of integration" are not better or worse from
/// > one another! You can do whatever you want, however you want!
/// > Consider the trade-offs of each approach, for example this example
/// > shows using observables as entities which means you need to be careful
/// > about type mismatches when subscribing to them!
/// > What I recommend is to only put observables on entities when it is
/// > related to that entity. Otherwise you can put them in resources or
/// > just use them directly from a system!
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin::default(),
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

/// Now the observable too is an entity!
/// This does change up a few things! Now you must use `Commands` to establish
/// subscriptions, and since those will execute later, there's no way to get
/// a direct reference to the actual subscription. Instead, the only thing
/// you get is the entity of the subscription which you can despawn to
/// unsubscribe it!
///
/// And the other, even more important thing is, is that the output types of
/// the observable are no longer there! It's just an `Entity`! So you have to
/// define them yourself, and it's **very easy to make mistakes** because it's
/// also very easy to change the output type of an observable by adding an
/// operator to it!
///
/// > Try uncommenting the `map` operator below to see what happens if the
/// > subscribe commands types (acquired from the EntityDestination) do not
/// > match the output types of any of the observables on the
/// > `observable_entity`!
/// > You get a very lengthy error message explaining what happened!
fn setup_subscription(
	mut commands: Commands,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
	let destination_entity = commands
		.spawn_empty()
		.observe(|signal: On<RxSignal<usize>>| println!("{:?}", signal.signal()))
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
