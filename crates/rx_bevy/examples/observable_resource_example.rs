use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
	time::common_conditions::on_timer,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{
	SubscriptionMapResource, print_notification_observer, send_message, toggle_subscription_system,
};
use rx_bevy::prelude::*;
use rx_bevy_observable_resource::observable::{ResourceObservable, ResourceObservableOptions};

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin::default(),
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
			RxSchedulerPlugin::<PostUpdate, Virtual>::default(),
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<ExampleEntities, usize, Never, PostUpdate, Virtual>(
					KeyCode::Space,
					|res| res.message_observable,
					|res| res.dummy_message_destination,
				),
				(
					dummy_resource_mutator.run_if(on_timer(Duration::from_millis(500))),
					init_dummy_resource.run_if(input_just_pressed(KeyCode::KeyR)),
				)
					.chain(),
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	subscriptions: HashMap<(Entity, Entity), Entity>,
	message_observable: Entity,
	dummy_message_destination: Entity,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct DummyResource {
	pub count: usize,
}

fn init_dummy_resource(mut commands: Commands) {
	commands.insert_resource::<DummyResource>(DummyResource { count: 0 });
}

fn dummy_resource_mutator(dummy_resource: Option<ResMut<DummyResource>>) {
	if let Some(mut resource) = dummy_resource {
		resource.count += 1;
		println!("Incrementing count to {resource:?}");
	}
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

fn setup(mut commands: Commands, rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
	println!("Press R to start mutating the resource!");
	println!("Press Space to subscribe!");
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let dummy_message_destination = commands
		.spawn(Name::new("ExampleObserver"))
		.observe(print_notification_observer::<usize, Never, Virtual>)
		.id();

	let message_observable = commands
		.spawn((
			Name::new("ResourceObservable"),
			ResourceObservable::<DummyResource, _, usize>::new(
				|res| res.count,
				ResourceObservableOptions {
					trigger_on_is_added: true, // If false, the first signal will be 1
					trigger_on_is_changed: true,
				},
				rx_schedule_update_virtual.handle(),
			)
			.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		dummy_message_destination,
		message_observable,
	});
}
