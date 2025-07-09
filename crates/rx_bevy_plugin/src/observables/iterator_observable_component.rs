use crate::{DebugBound, ObservableOnInsertContext, RxNext, RxTick, SubscriptionContext};
use crate::{
	ObservableComponent, ObservableSignalBound, ScheduledSubscription, on_observable_insert_hook,
	on_observable_remove_hook,
};
use bevy::ecs::component::{Mutable, StorageType};
use bevy::prelude::*;

use derive_where::derive_where;
use rx_bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]

pub struct IteratorObservableSubscriber<Iterator>
where
	Iterator: 'static + IntoIterator + Send + Sync,
	Iterator::Item: 'static + ObservableSignalBound,
{
	iterator: Iterator,
}

impl<Iterator> ObservableOutput for IteratorObservableSubscriber<Iterator>
where
	Iterator: 'static + IntoIterator + Send + Sync,
	Iterator::Item: 'static + ObservableSignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator> ScheduledSubscription for IteratorObservableSubscriber<Iterator>
where
	Iterator: 'static + IntoIterator + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const TICKABLE: bool = false;

	fn on_tick(&mut self, _event: &RxTick, _context: SubscriptionContext) {}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {}
}

#[derive(Clone, Reflect)]
#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + ObservableSignalBound,
{
	iterator: Iterator,
	/// One on One relationship, will spawn and despawn together
	subscribe_observer_entity: Option<Entity>,
}

/// TODO: Abstract this away, this is what makes an ObservableComponent Subscribable
impl<Iterator> Component for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(on_observable_insert_hook::<Self>);
		hooks.on_remove(on_observable_remove_hook::<Self>);
	}
}

impl<Iterator> IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + ObservableSignalBound,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator,
			subscribe_observer_entity: None,
		}
	}
}

impl<Iterator> ObservableComponent for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const CAN_SELF_SUBSCRIBE: bool = true;

	type ScheduledSubscription = IteratorObservableSubscriber<Iterator>;

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

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, _context: SubscriptionContext) -> Self::ScheduledSubscription {
		println!("on_subscribe iterator! {:?}", _context);

		for item in self.iterator.clone().into_iter() {
			_context
				.commands
				.trigger_targets(RxNext(item), _context.subscriber_entity);
		}

		// TODO: use instead if scheduled
		IteratorObservableSubscriber {
			iterator: self.iterator.clone(),
		}
	}
}

impl<Iterator> ObservableOutput for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + ObservableSignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}
