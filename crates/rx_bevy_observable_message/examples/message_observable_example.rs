use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{
	SubscriptionMapResource, print_notification_observer, send_message, toggle_subscription_system,
};
use rx_bevy::prelude::*;
use rx_bevy_observable_message::observable::MessageObservable;
use rx_core_traits::Never;

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
		.add_event::<DummyMessage>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<ExampleEntities, DummyMessage, Never, Update, Virtual>(
					KeyCode::Space,
					|res| res.message_observable,
					|res| res.dummy_message_destination,
				),
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				dummy_message_producer,
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

// TODO(bevy-0.17): Use Message
#[derive(Event, Debug, Clone)]
pub struct DummyMessage {
	pub count: usize,
}

fn dummy_message_producer(
	time: Res<Time>,
	mut dummy_message_writer: EventWriter<DummyMessage>,
	mut timer: Local<Timer>,
	mut setup: Local<bool>,
	mut count: Local<usize>,
) {
	if !*setup {
		timer.set_duration(Duration::from_millis(500));
		timer.set_mode(TimerMode::Repeating);
		timer.reset();
		*setup = true;
	}

	timer.tick(time.delta());

	if timer.just_finished() {
		let dummy_message = DummyMessage { count: *count };

		println!("Message written! {dummy_message:?}");
		dummy_message_writer.write(dummy_message);

		*count += 1;
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

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let dummy_message_destination = commands
		.spawn(Name::new("ExampleObserver"))
		.observe(print_notification_observer::<DummyMessage, Never>)
		.id();

	let message_observable = commands
		.spawn((
			Name::new("MessageObservable"),
			MessageObservable::<DummyMessage>::default().into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		dummy_message_destination,
		message_observable,
	});
}
