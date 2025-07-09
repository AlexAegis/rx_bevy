use bevy::prelude::*;
use bevy_ecs::component::{Mutable, StorageType};
use rx_bevy::ObservableOutput;

use std::time::Duration;

use crate::{
	ObservableComponent, RxNext, RxTick, ScheduledSubscription, SubscriptionContext,
	WithSubscribeObserverReference, on_observable_insert_hook, on_observable_remove_hook,
};

#[derive(Clone, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObservableOptions {
	pub duration: Duration,
	/// Whether or not the first emission, `0` should happen on subscribe
	/// or after the duration had elapsed once.
	pub start_on_subscribe: bool,
}

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
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(on_observable_insert_hook::<Self>);
		hooks.on_remove(on_observable_remove_hook::<Self>);
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

	fn on_insert(&mut self, _context: super::ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, context: super::SubscriptionContext) -> Self::Subscription {
		if self.options.start_on_subscribe {
			context
				.commands
				.trigger_targets(RxNext(0), context.subscriber_entity);
		}
		IntervalSubscription::new(self.options.clone())
	}
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalSubscription {
	timer: Timer,
	count: i32,
}

impl IntervalSubscription {
	pub fn new(interval_observable_options: IntervalObservableOptions) -> Self {
		Self {
			timer: Timer::new(interval_observable_options.duration, TimerMode::Repeating),
			count: if interval_observable_options.start_on_subscribe {
				1
			} else {
				0
			},
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
