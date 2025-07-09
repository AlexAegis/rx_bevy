use bevy::prelude::*;
use bevy_ecs::component::{Mutable, StorageType};
use rx_bevy::ObservableOutput;

use std::time::Duration;

use crate::{
	ObservableComponent, RxNext, RxTick, ScheduledSubscription, SubscriptionContext,
	on_observable_insert_hook, on_observable_remove_hook,
};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObservableComponent {
	subscribe_observer: Option<Entity>,
	duration: Duration,
}

impl IntervalObservableComponent {
	pub fn new(duration: Duration) -> Self {
		Self {
			duration,
			subscribe_observer: None,
		}
	}
}

impl ObservableOutput for IntervalObservableComponent {
	type Out = i32;
	type OutError = ();
}

impl Component for IntervalObservableComponent {
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(on_observable_insert_hook::<Self>);
		hooks.on_remove(on_observable_remove_hook::<Self>);
	}
}

impl ObservableComponent for IntervalObservableComponent {
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = IntervalSubscription;

	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer
	}

	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity> {
		self.subscribe_observer.replace(subscribe_observer_entity)
	}

	fn on_insert(&mut self, _context: super::ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, _context: super::SubscriptionContext) -> Self::Subscription {
		println!("interval on_subscribe {_context:?}");
		IntervalSubscription::new(self.duration.clone())
	}
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalSubscription {
	timer: Timer,
	count: i32,
}

impl IntervalSubscription {
	pub fn new(duration: Duration) -> Self {
		Self {
			timer: Timer::new(duration, TimerMode::Repeating),
			count: 0,
		}
	}
}

impl ObservableOutput for IntervalSubscription {
	type Out = i32;
	type OutError = ();
}

impl ScheduledSubscription for IntervalSubscription {
	fn on_tick(&mut self, event: &RxTick, context: SubscriptionContext) {
		self.timer.tick(event.delta);
		if self.timer.just_finished() {
			context
				.commands
				.trigger_targets(RxNext(self.count), context.subscriber_entity);
			self.count += 1;
		}
	}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {
		println!(
			"Interval unsubscribed! {}, {}",
			self.timer.elapsed_secs(),
			self.count
		);
	}
}
