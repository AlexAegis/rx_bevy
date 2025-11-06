use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{
	SubscriptionMapResource, print_notification_observer, send_event, toggle_subscription_system,
};
use rx_bevy::prelude::*;

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
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyW,
					|res| res.interval_observable,
					|res| res.subject_usize,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyE,
					|res| res.interval_observable,
					|res| res.replay_subject_usize,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyR,
					|res| res.interval_observable,
					|res| res.behavior_subject_usize,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyS,
					|res| res.subject_usize,
					|res| res.example_event_observer,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyD,
					|res| res.replay_subject_usize,
					|res| res.example_event_observer,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyF,
					|res| res.behavior_subject_usize,
					|res| res.example_event_observer,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyX,
					|res| res.subject_usize,
					|res| res.example_event_observer_2,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyC,
					|res| res.replay_subject_usize,
					|res| res.example_event_observer_2,
				),
				toggle_subscription_system::<ExampleEntities, usize, ()>(
					KeyCode::KeyV,
					|res| res.behavior_subject_usize,
					|res| res.example_event_observer_2,
				),
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	example_event_observer: Entity,
	example_event_observer_2: Entity,
	subscriptions: HashMap<(Entity, Entity), Entity>,
	keyboard_observable: Entity,
	subject_usize: Entity,
	replay_subject_usize: Entity,
	behavior_subject_usize: Entity,
	interval_observable: Entity,
}

impl SubscriptionMapResource for ExampleEntities {
	fn insert(
		&mut self,
		observable_destination_key: (Entity, Entity),
		subscription_entity: Entity,
	) {
		self.subscriptions
			.insert(observable_destination_key, subscription_entity);
	}

	fn remove(&mut self, observable_destination_key: (Entity, Entity)) -> Option<Entity> {
		self.subscriptions.remove(&observable_destination_key)
	}
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let example_event_observer = commands
		.spawn(Name::new("ExampleObserver"))
		.observe(print_notification_observer::<String>)
		.observe(print_notification_observer::<i32>)
		.observe(print_notification_observer::<usize>)
		.observe(print_notification_observer::<KeyCode>)
		.id();

	let example_event_observer_2 = commands
		.spawn(Name::new("ExampleObserver 2"))
		.observe(print_notification_observer::<usize>)
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

	/// TODO: Bug, crashes on despawn
	let subject_usize = commands
		.spawn((
			Name::new("Subject<usize>"),
			Subject::<usize, (), BevySubscriptionContextProvider>::default().into_component(),
		))
		.id();

	let replay_subject_usize = commands
		.spawn((
			Name::new("ReplaySubject<usize>"),
			ReplaySubject::<3, usize, (), BevySubscriptionContextProvider>::default()
				.into_component(),
		))
		.id();

	let behavior_subject_usize = commands
		.spawn((
			Name::new("BehaviorSubject<usize>"),
			BehaviorSubject::<usize, (), BevySubscriptionContextProvider>::new(0).into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		example_event_observer,
		example_event_observer_2,
		keyboard_observable,
		interval_observable,
		subject_usize,
		replay_subject_usize,
		behavior_subject_usize,
	});
}
