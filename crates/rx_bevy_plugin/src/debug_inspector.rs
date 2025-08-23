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
use short_type_name::short_type_name;

use crate::{
	ObservableComponent, OperatorComponent, RxSubscription, SignalBound, SubscriptionMarker,
	SubscriptionOf, SubscriptionSchedule, SubscriptionSignalDestination, SubscriptionSignalSources,
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

pub(crate) fn register_operator_debug_systems<Op>(deferred_world: &mut DeferredWorld)
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	let operator_debug_system_id = deferred_world
		.commands()
		.register_system(operator_entity_debug_print::<Op>);

	let subscription_debug_system_id = deferred_world
		.commands()
		.register_system(subscription_entity_debug_print::<Op::Subscriber>);

	let mut debug_registry = deferred_world
		.get_resource_mut::<DebugSystemRegistry>()
		.unwrap();

	debug_registry.debug_systems.push(operator_debug_system_id);
	debug_registry
		.debug_systems
		.push(subscription_debug_system_id);
}

pub(crate) fn operator_entity_debug_print<Op>(
	operator_query: Query<
		(
			Entity,
			Option<&SubscriptionSignalDestination<Op::Subscriber>>,
			Option<&SubscriptionSignalSources<Op::Subscriber>>,
			Option<&SubscriptionOf<Op::Subscriber>>,
			Option<&Subscriptions<Op::Subscriber>>,
			Option<&SubscriptionSchedule<Update>>,
		),
		With<Op>,
	>,
) where
	Op: OperatorComponent + 'static,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	for (
		entity,
		subscription_signal_destination,
		subscription_signal_sources,
		subscriber_instance_of,
		subscriber_instances,
		subscription_schedule,
	) in operator_query.iter()
	{
		println!("Operator Entity {entity:?} {}", short_type_name::<Op>());

		if let Some(d) = subscription_signal_destination {
			println!("{}", d);
		}
		if let Some(d) = subscription_signal_sources {
			println!("{}", d);
		}
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

pub(crate) fn observable_entity_debug_print<O>(
	observable_query: Query<
		(
			Entity,
			Option<&SubscriptionSignalDestination<O::Subscription>>,
			Option<&SubscriptionSignalSources<O::Subscription>>,
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
	for (
		entity,
		subscription_signal_destination,
		subscription_signal_sources,
		subscriber_instance_of,
		subscriber_instances,
		subscription_schedule,
	) in observable_query.iter()
	{
		println!("Observable Entity {entity:?} {}", short_type_name::<O>());

		if let Some(d) = subscription_signal_destination {
			println!("{}", d);
		}
		if let Some(d) = subscription_signal_sources {
			println!("{}", d);
		}
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

impl<Sub> Display for &SubscriptionSignalDestination<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSignal Destination: {}", self.get_destination())
	}
}

impl<Sub> Display for &SubscriptionSignalSources<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSignal Sources: {:?}", self.get_subscriptions())
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
			Option<&SubscriptionSignalDestination<Sub>>,
			Option<&SubscriptionSignalSources<Sub>>,
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
	for (
		entity,
		subscription_signal_destination,
		subscription_signal_sources,
		subscriber_instance_of,
		subscriber_instances,
		subscription_schedule,
	) in subscription_query.iter()
	{
		println!(
			"Subscription Entity {entity:?} {}",
			short_type_name::<Sub>()
		);

		if let Some(d) = subscription_signal_destination {
			println!("{}", d);
		}
		if let Some(d) = subscription_signal_sources {
			println!("{}", d);
		}
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
