use std::fmt::Display;

use bevy_app::{Plugin, Update};
use bevy_ecs::{
	entity::Entity,
	query::With,
	resource::Resource,
	schedule::IntoScheduleConfigs,
	system::{Commands, Query, Res, SystemId},
	world::DeferredWorld,
};
use bevy_input::{common_conditions::input_just_pressed, keyboard::KeyCode};
use bevy_log::debug;
use disqualified::ShortName;
use rx_core_traits::Observable;

use crate::{ObservableComponent, ObservableSubscriptions, SubscriptionOf};

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
	O: 'static + Observable + Send + Sync,
{
	let observable_debug_system_id = deferred_world
		.commands()
		.register_system(observable_entity_debug_print::<O>);

	let subscription_debug_system_id = deferred_world
		.commands()
		.register_system(subscription_entity_debug_print::<O>);

	if let Some(mut debug_registry) = deferred_world.get_resource_mut::<DebugSystemRegistry>() {
		debug_registry
			.debug_systems
			.push(observable_debug_system_id);
		debug_registry
			.debug_systems
			.push(subscription_debug_system_id);
	};
}

pub(crate) fn observable_entity_debug_print<O>(
	observable_query: Query<
		(
			Entity,
			Option<&SubscriptionOf<O>>,
			Option<&ObservableSubscriptions<O>>,
		),
		With<ObservableComponent<O>>,
	>,
) where
	O: 'static + Observable + Send + Sync,
{
	for (entity, subscriber_instance_of, subscriber_instances) in observable_query.iter() {
		debug!("Observable Entity {entity:?} {}", ShortName::of::<O>());

		if let Some(d) = subscriber_instance_of {
			debug!("{}", d);
		}
		if let Some(d) = subscriber_instances {
			debug!("{}", d);
		}
	}
}

impl<O> Display for &SubscriptionOf<O>
where
	O: 'static + Observable + Send + Sync,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscription of: {}", self.get_observable_entity())
	}
}

impl<O> Display for &ObservableSubscriptions<O>
where
	O: 'static + Observable + Send + Sync,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptions: {:?}", self.get_subscription_entities())
	}
}

pub(crate) fn subscription_entity_debug_print<O>(
	subscription_query: Query<(
		Entity,
		&SubscriptionOf<O>,
		Option<&ObservableSubscriptions<O>>,
	)>,
) where
	O: 'static + Observable + Send + Sync,
{
	for (entity, subscriber_instance_of, subscriber_instances) in subscription_query.iter() {
		debug!("Subscription Entity {entity:?} {}", ShortName::of::<O>());

		debug!("{}", subscriber_instance_of);

		if let Some(d) = subscriber_instances {
			debug!("{}", d);
		}
	}
}
