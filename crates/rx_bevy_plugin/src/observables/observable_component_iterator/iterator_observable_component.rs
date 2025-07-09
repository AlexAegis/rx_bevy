use crate::{
	DebugBound, IteratorSubscription, ObservableOnInsertContext, RxNext, SubscriptionContext,
	WithSubscribeObserverReference,
};
use crate::{
	ObservableComponent, ObservableSignalBound, observable_on_insert_hook,
	observable_on_remove_hook,
};
use bevy_ecs::{
	component::{Component, ComponentHooks, Mutable, StorageType},
	entity::Entity,
};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use rx_bevy_observable::ObservableOutput;

#[derive(Clone, Reflect)]
#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct IteratorObservableComponent<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	iterator: Iterator,
	/// One on One relationship, will spawn and despawn together
	subscribe_observer_entity: Option<Entity>,
}

/// TODO: Abstract this away, this is what makes an ObservableComponent Subscribable
impl<Iterator, const EMIT_ON_TICK: bool> Component
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<Self>);
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator,
			subscribe_observer_entity: None,
		}
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> WithSubscribeObserverReference
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer_entity
	}

	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity> {
		self.subscribe_observer_entity
			.replace(subscribe_observer_entity)
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableComponent
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = IteratorSubscription<Iterator, EMIT_ON_TICK>;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, _context: SubscriptionContext) -> Self::Subscription {
		println!("on_subscribe iterator! {:?}", _context);

		if !EMIT_ON_TICK {
			for item in self.iterator.clone().into_iter() {
				_context
					.commands
					.trigger_targets(RxNext(item), _context.subscriber_entity);
			}
		}

		IteratorSubscription::new(self.iterator.clone())
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableOutput
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}
