use std::fmt::Debug;

use std::time::Duration;

use bevy::{
	ecs::{entity::EntityHashMap, schedule::ScheduleConfigs},
	input::common_conditions::input_just_pressed,
	prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_alternate_system_on_press::alternate_systems_on_press;
use examples_common::send_event;
use rx_bevy::prelude::*;
use short_type_name::short_type_name;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<KeyCode, ()>(KeyCode::KeyK, |e| e.keyboard_observable),
				toggle_subscription_system::<usize, ()>(KeyCode::KeyI, |e| e.interval_observable),
				toggle_subscription_system::<String, ()>(KeyCode::KeyL, |e| {
					e.keyboard_switch_map_to_interval_observable
				}),
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

fn print_next_observer<T>(mut next: Trigger<RxSignal<T>>, name_query: Query<&Name>, time: Res<Time>)
where
	T: SignalBound + Debug,
{
	println!(
		"{}\t value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		short_type_name::<T>(),
		next.event_mut().consume(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn toggle_subscription_system<Out: SignalBound, OutError: SignalBound>(
	toggle_key_code: KeyCode,
	observable_selector: impl Fn(&ResMut<ExampleEntities>) -> Entity + Send + Sync + 'static + Clone,
) -> (
	ScheduleConfigs<Box<dyn bevy::prelude::System<In = (), Out = Result<(), BevyError>> + 'static>>,
	ScheduleConfigs<Box<dyn bevy::prelude::System<In = (), Out = Result<(), BevyError>> + 'static>>,
) {
	let observable_selector_clone = observable_selector.clone();

	alternate_systems_on_press(
		toggle_key_code,
		subscribe_entity::<Out, OutError>(move |e| observable_selector_clone(e)),
		unsubscribe_entity(move |e| observable_selector(e)),
	)
}

fn subscribe_entity<Out, OutError>(
	observable_selector: impl Fn(&ResMut<ExampleEntities>) -> Entity,
) -> impl FnMut(Commands, ResMut<ExampleEntities>)
where
	Out: SignalBound,
	OutError: SignalBound,
{
	move |mut commands: Commands, mut example_entities: ResMut<ExampleEntities>| {
		let observable_entity = observable_selector(&example_entities);
		println!("subscribing to {}...", observable_entity);
		let subscription_entity = commands.subscribe::<Out, OutError, Update>(
			observable_selector(&example_entities),
			example_entities.example_event_observer,
		);
		println!(
			"subscription to {} was spawned as {}!",
			observable_entity, subscription_entity
		);

		example_entities
			.subscriptions
			.insert(observable_entity, subscription_entity);
	}
}

fn unsubscribe_entity(
	observable_selector: impl Fn(&ResMut<ExampleEntities>) -> Entity,
) -> impl FnMut(Commands, ResMut<ExampleEntities>) {
	move |mut commands: Commands, mut example_entities: ResMut<ExampleEntities>| {
		let observable_entity = observable_selector(&example_entities);
		if let Some(subscription_entity) = example_entities.subscriptions.remove(&observable_entity)
		{
			println!(
				"unsubscribing {} observables {} subscription...",
				observable_entity, subscription_entity
			);
			commands.unsubscribe(subscription_entity);
		} else {
			println!(
				"{} does not have an active subscription!",
				observable_entity
			);
		}
	}
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	example_event_observer: Entity,
	subscriptions: EntityHashMap<Entity>,
	keyboard_observable: Entity,
	keyboard_switch_map_to_interval_observable: Entity,
	interval_observable: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let example_event_observer = commands
		.spawn(Name::new("ExampleObserver"))
		.observe(print_next_observer::<String>)
		.observe(print_next_observer::<i32>)
		.observe(print_next_observer::<usize>)
		.observe(print_next_observer::<KeyCode>)
		.id();

	let keyboard_observable = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::default().into_component(),
		))
		.id();

	let interval_observable = commands
		.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(IntervalObservableOptions {
				duration: Duration::from_millis(500),
				start_on_subscribe: true,
				max_emissions_per_tick: 2,
			})
			.into_component(),
		))
		.id();

	let keyboard_switch_map_to_interval_observable = commands
		.spawn((
			Name::new("KeyboardSwitchMapToIntervalObservable"),
			KeyboardObservable::default()
				.filter(|key_code| {
					matches!(
						key_code,
						KeyCode::KeyW | KeyCode::KeyA | KeyCode::KeyS | KeyCode::KeyD
					)
				})
				.switch_map(|key_code| {
					// TODO: SwitchMap is unresponsive!!!
					let duration = match key_code {
						KeyCode::KeyW => Duration::from_millis(5),
						KeyCode::KeyA => Duration::from_millis(100),
						KeyCode::KeyS => Duration::from_millis(500),
						KeyCode::KeyD => Duration::from_millis(2000),
						_ => Duration::from_millis(500),
					};
					IntervalObservable::new(IntervalObservableOptions {
						duration,
						start_on_subscribe: true,
						max_emissions_per_tick: 4,
					})
				})
				.map(|key_code| format!("Ticking! {:?}", key_code))
				.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: EntityHashMap::new(),
		example_event_observer,
		keyboard_observable,
		interval_observable,
		keyboard_switch_map_to_interval_observable,
	});
}
