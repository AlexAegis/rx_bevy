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
use disqualified::ShortName;
use rx_bevy_common::Clock;
use rx_core_traits::Observable;

use crate::{
	ObservableComponent, ObservableSubscriptions, RxBevyContext, SubscriptionOf,
	SubscriptionSchedule,
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

pub(crate) fn register_observable_debug_systems<O, S, C>(deferred_world: &mut DeferredWorld)
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
	S: ScheduleLabel,
	C: Clock,
{
	let observable_debug_system_id = deferred_world
		.commands()
		.register_system(observable_entity_debug_print::<O, S, C>);

	let subscription_debug_system_id = deferred_world
		.commands()
		.register_system(subscription_entity_debug_print::<O, S, C>);

	if let Some(mut debug_registry) = deferred_world.get_resource_mut::<DebugSystemRegistry>() {
		debug_registry
			.debug_systems
			.push(observable_debug_system_id);
		debug_registry
			.debug_systems
			.push(subscription_debug_system_id);
	};
}

pub(crate) fn observable_entity_debug_print<O, S, C>(
	observable_query: Query<
		(
			Entity,
			Option<&SubscriptionOf<O>>,
			Option<&ObservableSubscriptions<O>>,
			Option<&SubscriptionSchedule<S, C>>,
		),
		With<ObservableComponent<O>>,
	>,
) where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
	S: ScheduleLabel,
	C: Clock,
{
	for (entity, subscriber_instance_of, subscriber_instances, subscription_schedule) in
		observable_query.iter()
	{
		println!("Observable Entity {entity:?} {}", ShortName::of::<O>());

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

impl<O> Display for &SubscriptionOf<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscription of: {}", self.get_observable_entity())
	}
}

impl<O> Display for &ObservableSubscriptions<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptions: {:?}", self.get_subscription_entities())
	}
}

impl<S, C> Display for &SubscriptionSchedule<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSubscriptionSchedule<{}>", ShortName::of::<S>())
	}
}

pub(crate) fn subscription_entity_debug_print<O, S, C>(
	subscription_query: Query<(
		Entity,
		&SubscriptionOf<O>,
		Option<&ObservableSubscriptions<O>>,
		Option<&SubscriptionSchedule<S, C>>,
	)>,
) where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
	S: ScheduleLabel,
	C: Clock,
{
	for (entity, subscriber_instance_of, subscriber_instances, subscription_schedule) in
		subscription_query.iter()
	{
		println!("Subscription Entity {entity:?} {}", ShortName::of::<O>());

		println!("{}", subscriber_instance_of);

		if let Some(d) = subscriber_instances {
			println!("{}", d);
		}
		if let Some(d) = subscription_schedule {
			println!("{}", d);
		}
	}
}
