use core::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{
	Never, Observer, ObserverNotification, Scheduler, SchedulerHandle,
	SchedulerScheduleTaskExtension, Signal, TaskCancellationId, TeardownCollectionExtension,
	UpgradeableObserver,
};

use crate::{DetachedSubscriber, RxBevyScheduler};

/// This is not a component, but a wrapper for an Entity to be used as a generic
/// destination for subscriptions. The entity here will receive all signals as
/// [ConsumableSubscriberNotificationEvent][crate::ConsumableSubscriberNotificationEvent]'s.
///
/// It's mainly used by user made subscriptions, whenever you make a subscription
/// through [Commands][bevy_ecs::Commands], the destination entity will be
/// wrapped into this one.
///
/// > Technically this is an Observer in the Rx terms and should be called
/// > `EntityObserver` but that would be a very confusing term in Bevy.
/// > And while most, simple observers do not
#[derive(RxObserver, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_does_not_upgrade_to_observer_subscriber]
pub struct EntityDestination<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	destination: Entity,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	owner_id: TaskCancellationId,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(destination: Entity, mut scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		let owner_id = scheduler.lock().generate_cancellation_id();
		Self {
			destination,
			scheduler,
			owner_id,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> UpgradeableObserver for EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		let owner_id = self.owner_id.clone();
		let mut scheduler = self.scheduler.clone();
		let mut upgraded = DetachedSubscriber::new(self);
		upgraded.add_fn(move || {
			scheduler.lock().cancel(owner_id);
		});
		upgraded
	}
}

impl<In, InError> Observer for EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: Self::In) {
		let destination = self.destination;
		self.scheduler.lock().schedule_immediate_task(
			move |_, context| {
				context.send_observer_notification(
					destination,
					ObserverNotification::<In, InError>::Next(next),
				);
			},
			self.owner_id,
		);
	}

	fn error(&mut self, error: Self::InError) {
		let destination = self.destination;
		self.scheduler.lock().schedule_immediate_task(
			move |_, context| {
				context.send_observer_notification(
					destination,
					ObserverNotification::<In, InError>::Error(error),
				);
			},
			self.owner_id,
		);
	}

	fn complete(&mut self) {
		let destination = self.destination;
		self.scheduler.lock().schedule_immediate_task(
			move |_, context| {
				context.send_observer_notification(
					destination,
					ObserverNotification::<In, InError>::Complete,
				);
			},
			self.owner_id,
		);
	}
}
