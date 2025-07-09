use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::Observer,
	system::Commands,
	world::DeferredWorld,
};
use bevy_log::debug;
use derive_where::derive_where;

use rx_bevy_observable::ObservableOutput;
use short_type_name::short_type_name;

use crate::{
	DebugBound, ObservableSignalBound, ScheduledSubscription, SubscribeObserverComponent,
	SubscriptionContext, on_subscribe,
};

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
pub trait ObservableComponent:
	ObservableOutput + Component<Mutability = Mutable> + WithSubscribeObserverReference + DebugBound
where
	Self::Out: Send + Sync + DebugBound,
	Self::OutError: Send + Sync + DebugBound,
{
	const CAN_SELF_SUBSCRIBE: bool;

	/// If the Observable does not need any scheduling, use [NonScheduledSubscription]
	/// Otherwise implement a [ScheduledSubscription] that can emit events when
	/// ticked by an [RxScheduler].
	type Subscription: ScheduledSubscription<Out = Self::Out, OutError = Self::OutError>
		+ Send
		+ Sync;

	fn on_insert(&mut self, context: ObservableOnInsertContext);

	fn on_subscribe(&mut self, context: SubscriptionContext) -> Self::Subscription;
}

/// TODO: While this is required for all ObservableComponents, it's a separate trait to be the auto-implementable by a macro.
///
/// This is technically a one-on-one relationship, each ObservableComponent has
/// exactly one other entity listening for [Subscribe] events
pub trait WithSubscribeObserverReference {
	/// Should return the entity reference to the entity that observes [Subscribe]
	/// events for this observable
	fn get_subscribe_observer_entity(&self) -> Option<Entity>;

	/// Returns the previous observer entity, if exists.
	/// (Implement as `.replace` on the stored `Option<Entity>`)
	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity>;
}

#[derive_where(Debug)]
pub struct ObservableOnInsertContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
}

/// This on_insert hook sets up the observable so it can spawn new subscriptions
/// upon receiving [Subscribe] events.
pub fn observable_on_insert_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = hook_context.entity;

	// This is the observer that processes [Subscribe] events for this specific observable.
	// It will be despawned when the observable is removed.
	let subscribe_observer_entity = {
		let mut commands = deferred_world.commands();
		debug!(
			"setting up subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);

		commands
			.spawn((
				ChildOf(observable_entity), // Purely for organizational purposes in debug views like WorldInspector
				SubscribeObserverComponent::<O>::new(observable_entity),
				Name::new(format!(
					"Observer (Observable Subscribe) - {}({}) ",
					short_type_name::<O>(),
					observable_entity
				)),
				Observer::new(on_subscribe::<O>).with_entity(observable_entity),
			))
			.id()
	};

	{
		let (mut entities, mut commands) = deferred_world.entities_and_commands();
		let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();

		let mut component = observable_entity_mut.get_mut::<O>().unwrap();
		component.set_subscribe_observer_entity(subscribe_observer_entity);

		component.on_insert(ObservableOnInsertContext {
			observable_entity,
			commands: &mut commands,
		});
	}
}

/// To achieve a one-on-one relationship, the observer that observes [Subscribe] events
/// is despawned when the observable component is removed
pub fn observable_on_remove_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = hook_context.entity;
	let (mut entities, mut commands) = deferred_world.entities_and_commands();
	let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();
	let observable_component = observable_entity_mut.get_mut::<O>().unwrap();

	if let Some(subscribe_observer_entity) = observable_component.get_subscribe_observer_entity() {
		debug!(
			"despawning subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);
		commands.entity(subscribe_observer_entity).despawn();
	}
}
