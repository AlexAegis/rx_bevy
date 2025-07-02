use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rx_bevy_plugin::{
	EntityCommandObservableExtension, EntityObserver, EntitySubscriptionDestination,
	InternalEntityObserver, IteratorObservableComponent, OnNext,
};

/// This test showcases in what order observables execute their observers
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.add_systems(Startup, setup)
		.run()
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let observable_entity = commands
		.spawn((
			Name::new("IteratorObservable"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			IteratorObservableComponent::new(1..=10),
		))
		.subscribe(EntityObserver {
			// Mybe an On<Subscribe> event handler/observer to define how a subscription/pipeline is made??
			destination: EntitySubscriptionDestination::This,
			/// TODO:This probably should just define an event type and use a reguler observer system with On<Next<Signal>>
			on_next_system: next_number_observer,
		})
		.id();

	let observer_entity = commands
		.spawn((
			Name::new("IteratorObservable"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		))
		.subscribe(EntitySubscriptionDestination::Other(observable_entity))
		.id();
}

#[derive(Event, Debug)]
pub struct NumberSignal {
	value: i32,
}

/// TODO: Trigger<Next<NumberSignal>> ?
fn next_number_observer(next: OnNext<NumberSignal>) {
	println!("value observed: {:?}", next.value);
}
