use bevy_ecs::{
	component::{Component, ComponentHooks, Mutable, StorageType},
	entity::Entity,
};
use bevy_reflect::Reflect;
use rx_bevy_observable::ObservableOutput;

use crate::{
	IntervalObservableOptions, IntervalSubscription, ObservableComponent,
	ObservableOnInsertContext, RxNext, SubscriptionContext, WithSubscribeObserverReference,
	observable_on_insert_hook, observable_on_remove_hook,
};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObservableComponent {
	options: IntervalObservableOptions,
	subscribe_observer: Option<Entity>,
}

impl IntervalObservableComponent {
	pub fn new(options: IntervalObservableOptions) -> Self {
		Self {
			options,
			subscribe_observer: None,
		}
	}
}

impl ObservableOutput for IntervalObservableComponent {
	type Out = i32;
	type OutError = ();
}

impl Component for IntervalObservableComponent {
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<Self>);
	}
}

impl WithSubscribeObserverReference for IntervalObservableComponent {
	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer
	}

	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity> {
		self.subscribe_observer.replace(subscribe_observer_entity)
	}
}

impl ObservableComponent for IntervalObservableComponent {
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = IntervalSubscription;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, context: SubscriptionContext) -> Self::Subscription {
		if self.options.start_on_subscribe {
			context
				.commands
				.trigger_targets(RxNext(0), context.subscriber_entity);
		}
		IntervalSubscription::new(self.options.clone())
	}
}
