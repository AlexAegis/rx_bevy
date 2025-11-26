use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{print_notification_observer, send_message};
use rx_bevy::prelude::*;
use rx_core_traits::Never;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			//  EguiPlugin::default(), TODO(bevy-0.17): EguiPlugin::default()
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
				despawn_instant_subscription.run_if(input_just_pressed(KeyCode::KeyQ)),
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

fn despawn_instant_subscription(mut commands: Commands, r: Res<ExampleEntities>) {
	commands.entity(r.ad_hoc_subscription_entity).despawn();
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	destination_entity: Entity,
	ad_hoc_subscription_entity: Entity,
	ad_hoc_subscription_2_entity: Entity,
}

fn setup(mut commands: Commands, mut context: RxBevyContextItem) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let destination_entity = commands
		.spawn(Name::new("Destination"))
		.observe(print_notification_observer::<usize, Never, Virtual>)
		.id();

	let ad_hoc_subscription = commands
		.with_observable::<_, Update, Virtual>(IntervalObservable::new(IntervalObservableOptions {
			duration: Duration::from_millis(400),
			start_on_subscribe: true,
			max_emissions_per_tick: 2,
		}))
		.filter(|next| next % 2 == 1)
		.subscribe(EntityDestination::new(destination_entity), &mut context);

	let ad_hoc_subscription_2 = IntervalObservable::new(IntervalObservableOptions {
		duration: Duration::from_millis(200),
		start_on_subscribe: true,
		max_emissions_per_tick: 2,
	})
	.with_commands::<Update, Virtual>(commands.reborrow())
	.filter(|next| next % 2 == 0)
	.subscribe(EntityDestination::new(destination_entity), &mut context);

	commands.insert_resource(ExampleEntities {
		destination_entity,
		ad_hoc_subscription_entity: ad_hoc_subscription.into(),
		ad_hoc_subscription_2_entity: ad_hoc_subscription_2.into(),
	});
}
