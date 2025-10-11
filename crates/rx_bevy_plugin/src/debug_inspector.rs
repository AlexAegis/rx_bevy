use std::fmt::Display;

use bevy_app::{Plugin, Update};
use bevy_ecs::{
	entity::Entity,
	query::With,
	resource::Resource,
	schedule::{IntoScheduleConfigs, ScheduleLabel},
	system::{Commands, Query, Res, SystemId},
	world::DeferredWorld,
};
use bevy_input::{common_conditions::input_just_pressed, keyboard::KeyCode};
use rx_bevy_core::SignalBound;
use short_type_name::short_type_name;

use crate::{
	ObservableComponent, RxSubscription, SubscriptionMarker, SubscriptionOf, SubscriptionSchedule,
	Subscriptions,
};

pub struct DebugInspectorPlugin;

impl Plugin for DebugInspectorPlugin {
	fn build(&self, app: &mut bevy_app::App) {
		app.init_resource::<DebugSystemRegistry>();
		app.add_systems(
			Update,
			run_debug_systems.run_if(input_just_pressed(KeyCode::KeyD)),
		);
	}
}

#[derive(Resource, Default)]
pub struct DebugSystemRegistry {
	pub debug_systems: Vec<SystemId>,
}

pub(crate) fn run_debug_systems(
	mut commands: Commands,
	debug_system_registry: Res<DebugSystemRegistry>,
) {
	for debug_system in debug_system_registry.debug_systems.iter() {
		commands.run_system(*debug_system);
	}
}

pub(crate) fn register_observable_debug_systems<O>(deferred_world: &mut DeferredWorld)
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	let observable_debug_system_id = deferred_world
		.commands()
		.register_system(observable_entity_debug_print::<O>);

	let subscription_debug_system_id = deferred_world
		.commands()
		.register_system(subscription_entity_debug_print::<O::Subscription>);

	let mut debug_registry = deferred_world
		.get_resource_mut::<DebugSystemRegistry>()
		.unwrap();

	debug_registry
		.debug_systems
		.push(observable_debug_system_id);
	debug_registry
		.debug_systems
		.push(subscription_debug_system_id);
}

pub(crate) fn observable_entity_debug_print<O>(
	observable_query: Query<
		(
			Entity,
			Option<&SubscriptionOf<O::Subscription>>,
			Option<&Subscriptions<O::Subscription>>,
			Option<&SubscriptionSchedule<Update>>,
		),
		With<O>,
	>,
) where
	O: ObservableComponent + 'static,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	for (entity, subscriber_instance_of, subscriber_instances, subscription_schedule) in
		observable_query.iter()
	{
		println!("Observable Entity {entity:?} {}", short_type_name::<O>());

		if let Some(d) = subscriber_instance_of {
			println!("{}", d);
		}
		if let Some(d) = subscriber_instances {
			println!("{}", d);
		}
		if let Some(d) = subscription_schedule {
			println!("{}", d);
		}
	}
}

impl<Sub> Display for &SubscriptionOf<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscription of: {}", self.get_instance_of())
	}
}

impl<Sub> Display for &Subscriptions<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptions: {:?}", self.get_instances())
	}
}

impl<S> Display for &SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptionSchedule<{}>", short_type_name::<S>())
	}
}

pub(crate) fn subscription_entity_debug_print<Sub>(
	subscription_query: Query<
		(
			Entity,
			Option<&SubscriptionOf<Sub>>,
			Option<&Subscriptions<Sub>>,
			Option<&SubscriptionSchedule<Update>>,
		),
		With<SubscriptionMarker>,
	>,
) where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	for (entity, subscriber_instance_of, subscriber_instances, subscription_schedule) in
		subscription_query.iter()
	{
		println!(
			"Subscription Entity {entity:?} {}",
			short_type_name::<Sub>()
		);

		if let Some(d) = subscriber_instance_of {
			println!("{}", d);
		}
		if let Some(d) = subscriber_instances {
			println!("{}", d);
		}
		if let Some(d) = subscription_schedule {
			println!("{}", d);
		}
	}
}
