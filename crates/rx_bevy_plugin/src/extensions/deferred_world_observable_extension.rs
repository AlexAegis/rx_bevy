use bevy_ecs::{
	component::{Component, Mutable},
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::Observer,
	world::DeferredWorld,
};

use crate::{
	ObservableComponent, ObservableOnInsertContext, OnInsertSubHook, OperatorComponent,
	OperatorSubscribeObserverOf, SignalBound, SubscribeObserverOf,
	default_on_subscribe_error_handler, on_observable_subscribe, on_operator_subscribe,
};
use short_type_name::short_type_name;

pub(crate) trait DeferredWorldObservableCallOnInsertExtension {
	fn call_on_insert_hook<O>(&mut self, observable_entity: Entity)
	where
		O: OnInsertSubHook + Component<Mutability = Mutable>;
}

#[cfg(feature = "reflect")]
pub(crate) trait DeferredWorldObservableRegisterSubscriptionTypesExtension {
	fn register_subscription_types<Sub>(&mut self)
	where
		Sub: crate::RxSubscription,
		Sub::Out: crate::SignalBound,
		Sub::OutError: crate::SignalBound;
}

pub(crate) trait DeferredWorldObservableSpawnObservableSubscribeObserverExtension {
	fn spawn_observable_subscribe_observer<O>(&mut self, observable_entity: Entity)
	where
		O: ObservableComponent + Send + Sync,
		O::Out: SignalBound,
		O::OutError: SignalBound;
}

pub(crate) trait DeferredWorldObservableSpawnOperatorSubscribeObserverExtension {
	fn spawn_operator_subscribe_observer<Op>(&mut self, observable_entity: Entity)
	where
		Op: OperatorComponent + Send + Sync,
		Op::In: SignalBound,
		Op::InError: SignalBound,
		Op::Out: SignalBound,
		Op::OutError: SignalBound;
}

impl DeferredWorldObservableCallOnInsertExtension for DeferredWorld<'_> {
	fn call_on_insert_hook<O>(&mut self, entity: Entity)
	where
		O: OnInsertSubHook + Component<Mutability = Mutable>,
	{
		let (mut entities, mut commands) = self.entities_and_commands();
		let mut observable_entity_mut = entities.get_mut(entity).unwrap();

		let mut component = observable_entity_mut.get_mut::<O>().unwrap();

		component.on_insert(ObservableOnInsertContext {
			observable_entity: entity,
			commands: &mut commands,
		});
	}
}

#[cfg(feature = "reflect")]
impl DeferredWorldObservableRegisterSubscriptionTypesExtension for DeferredWorld<'_> {
	fn register_subscription_types<Sub>(&mut self)
	where
		Sub: crate::RxSubscription,
		Sub::Out: crate::SignalBound,
		Sub::OutError: crate::SignalBound,
	{
		use bevy_ecs::reflect::AppTypeRegistry;

		let reg = self.resource_mut::<AppTypeRegistry>();
		let mut registry_lock = reg.write();

		registry_lock.register::<crate::SubscriptionOf<Sub>>();
		registry_lock.register::<crate::Subscriptions<Sub>>();
		registry_lock.register::<crate::SubscriptionSignalDestination<Sub>>();
		registry_lock.register::<crate::SubscriptionSignalSources<Sub>>();
	}
}
impl DeferredWorldObservableSpawnObservableSubscribeObserverExtension for DeferredWorld<'_> {
	fn spawn_observable_subscribe_observer<O>(&mut self, observable_entity: Entity)
	where
		O: ObservableComponent + Send + Sync,
		O::Out: SignalBound,
		O::OutError: SignalBound,
	{
		self.commands().spawn((
			SubscribeObserverOf::<O>::new(observable_entity),
			Observer::new(on_observable_subscribe::<O>)
				.with_entity(observable_entity)
				.with_error_handler(default_on_subscribe_error_handler),
			// TODO: Having this here is unnecessary and is causing a warning on despawn because of the double relationship. I'll leave this here for now just so the inspector is a little more organized until that too has a convenient method to register relationships
			ChildOf(observable_entity), // For organizational purposes in debug views like WorldInspector
			Name::new(format!(
				"Observer (Subscribe) - {}({}) ",
				short_type_name::<O>(),
				observable_entity
			)),
		));
	}
}

impl DeferredWorldObservableSpawnOperatorSubscribeObserverExtension for DeferredWorld<'_> {
	fn spawn_operator_subscribe_observer<Op>(&mut self, operator_entity: Entity)
	where
		Op: OperatorComponent + Send + Sync,
		Op::In: SignalBound,
		Op::InError: SignalBound,
		Op::Out: SignalBound,
		Op::OutError: SignalBound,
	{
		self.commands().spawn((
			OperatorSubscribeObserverOf::<Op>::new(operator_entity),
			Observer::new(on_operator_subscribe::<Op>)
				.with_entity(operator_entity)
				.with_error_handler(default_on_subscribe_error_handler),
			// TODO: Having this here is unnecessary and is causing a warning on despawn because of the double relationship. I'll leave this here for now just so the inspector is a little more organized until that too has a convenient method to register relationships
			ChildOf(operator_entity), // For organizational purposes in debug views like WorldInspector
			Name::new(format!(
				"Observer (Subscribe) - {}({}) ",
				short_type_name::<Op>(),
				operator_entity
			)),
		));
	}
}
