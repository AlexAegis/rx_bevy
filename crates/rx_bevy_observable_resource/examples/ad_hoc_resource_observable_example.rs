use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
	time::common_conditions::on_timer,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{print_notification_observer, send_message};
use rx_bevy::prelude::*;
use rx_bevy_observable_resource::observable::{ResourceObservable, ResourceObservableOptions};
use rx_core_traits::Never;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxScheduler::<Update, Virtual>::default(),
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
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
	dummy_message_destination: Entity,
	ad_hoc_resource_subscription: Entity,
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

fn setup(mut commands: Commands, mut context: RxBevyContextItem) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let dummy_message_destination = commands
		.spawn(Name::new("ExampleObserver"))
		.observe(print_notification_observer::<usize, Never, Virtual>)
		.id();

	// Store this somewhere, or mark it with a component, or you won't be able
	// to stop it!
	let ad_hoc_resource_subscription: Entity =
		ResourceObservable::<DummyResource, _, usize, Never>::new(
			|res| Ok(res.count),
			ResourceObservableOptions {
				trigger_on_is_added: true, // If false, the first signal will be 1
				trigger_on_is_changed: true,
			},
		)
		.with_commands::<Update, Virtual>(commands.reborrow())
		.subscribe(
			EntityDestination::new(dummy_message_destination),
			&mut context,
		)
		.into();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		dummy_message_destination,
		ad_hoc_resource_subscription,
	});
}
