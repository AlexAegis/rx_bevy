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
use rx_core_traits::Observable;
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContextProvider, EntitySubscriptionContextAccessProvider, ObservableComponent,
	ObservableSubscriptions, SubscriptionOf, SubscriptionSchedule,
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

pub(crate) fn register_observable_debug_systems<O, ContextAccess>(
	deferred_world: &mut DeferredWorld,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	let observable_debug_system_id = deferred_world
		.commands()
		.register_system(observable_entity_debug_print::<O, ContextAccess>);

	let subscription_debug_system_id = deferred_world
		.commands()
		.register_system(subscription_entity_debug_print::<O, ContextAccess>);

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

pub(crate) fn observable_entity_debug_print<O, ContextAccess>(
	observable_query: Query<
		(
			Entity,
			Option<&SubscriptionOf<O, ContextAccess>>,
			Option<&ObservableSubscriptions<O, ContextAccess>>,
			Option<&SubscriptionSchedule<Update>>,
		),
		With<ObservableComponent<O, ContextAccess>>,
	>,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
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

impl<O, ContextAccess> Display for &SubscriptionOf<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscription of: {}", self.get_observable_entity())
	}
}

impl<O, ContextAccess> Display for &ObservableSubscriptions<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptions: {:?}", self.get_subscription_entities())
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

pub(crate) fn subscription_entity_debug_print<O, ContextAccess>(
	subscription_query: Query<(
		Entity,
		&SubscriptionOf<O, ContextAccess>,
		Option<&ObservableSubscriptions<O, ContextAccess>>,
		Option<&SubscriptionSchedule<Update>>,
	)>,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	for (entity, subscriber_instance_of, subscriber_instances, subscription_schedule) in
		subscription_query.iter()
	{
		println!("Subscription Entity {entity:?} {}", short_type_name::<O>());

		println!("{}", subscriber_instance_of);

		if let Some(d) = subscriber_instances {
			println!("{}", d);
		}
		if let Some(d) = subscription_schedule {
			println!("{}", d);
		}
	}
}
