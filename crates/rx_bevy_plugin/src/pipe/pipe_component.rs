use bevy::prelude::*;
use bevy_ecs::component::{Mutable, StorageType};
use rx_bevy::{ObservableOutput, ObserverInput, Operator};

use crate::{
	ObservableComponent, ObservableOnInsertContext, ObservableOnSubscribeContext, RxNext,
	on_observable_insert_hook, on_observable_remove_hook,
};
/*
#[derive(Debug)]
pub struct PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	subscribe_observer_entity: Option<Entity>,
	operator: Op,
}

impl<Op> Component for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(on_observable_insert_hook::<Self>);
		hooks.on_remove(on_observable_remove_hook::<Self>);
	}
}

impl<Op> ObserverInput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	type In = Op::In;
	type InError = Op::InError;
}

impl<Op> ObservableOutput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Op> ObservableComponent for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscriber = ();

	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer_entity
	}

	fn set_subscribe_observer_entity(&mut self, subscribe_observer_entity: Entity) {
		self.subscribe_observer_entity = Some(subscribe_observer_entity);
	}

	fn on_insert(&mut self, context: ObservableOnInsertContext) {
		context
			.commands
			.entity(context.observable_entity)
			.observe(pipe_next_observer::<Op>);
	}

	/// The subscription creates a new observer entity, that entity should have the subscriber on it.
	fn on_subscribe(&mut self, _subscription_context: ObservableOnSubscribeContext) {
		//self.operator.operator_subscribe(destination)
	}
}

fn pipe_next_observer<Op>(
	trigger: Trigger<RxNext<Op::In>>,
	mut subject_query: Query<(&PipeComponent<Op>,)>,
	mut commands: Commands,
) where
	Op: 'static + Operator + Send + Sync,
	Op::In: Send + Sync,
	Op::InError: Send + Sync,
	Op::Out: Send + Sync,
	Op::OutError: Send + Sync,
{
	let Ok((pipe,)) = subject_query.get_mut(trigger.target()) else {
		return;
	};

	//commands.trigger_targets(
	//	trigger.event().clone(),
	//	subscription.get_subscriber_entities(),
	//);
}
*/
