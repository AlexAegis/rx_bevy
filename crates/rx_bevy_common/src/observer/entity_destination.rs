use core::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_common::{
	Never, ObserverNotification, RxObserver, Scheduler, SchedulerHandle,
	SchedulerScheduleWorkExtension, SharedSubscription, Signal, SubscriptionLike,
	WorkCancellationId,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::RxBevyScheduler;

/// This is not a component, but a wrapper for an Entity to be used as a generic
/// destination for subscriptions. The entity here will receive all signals as
/// an [RxSignal][crate::RxSignal] event.
///
/// It's mainly used by user-made subscriptions. Whenever you make a
/// subscription through [Commands][bevy_ecs::Commands], the destination entity
/// will be wrapped into this one.
///
/// > Technically this is an Observer in Rx terms and should be called
/// > `EntityObserver` but that would be very confusing in Bevy.
#[derive(RxSubscriber, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
pub struct EntityDestination<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	destination: Entity,
	#[teardown]
	teardown: SharedSubscription,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	cancellation_id: WorkCancellationId,
	_phantom_data: PhantomData<fn(In, InError) -> (In, InError)>,
}

impl<In, InError> EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(destination: Entity, scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();
		Self {
			destination,
			scheduler,
			teardown: SharedSubscription::default(),
			cancellation_id,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> RxObserver for EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: Self::In) {
		let destination = self.destination;
		let mut teardown = self.teardown.clone();
		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if context.deferred_world.get_entity(destination).is_ok() {
					context.send_observer_notification(
						destination,
						ObserverNotification::<In, InError>::Next(next),
					);
				} else {
					teardown.unsubscribe();
				}
			},
			self.cancellation_id,
		);
	}

	fn error(&mut self, error: Self::InError) {
		let destination = self.destination;

		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if context.deferred_world.get_entity(destination).is_ok() {
					context.send_observer_notification(
						destination,
						ObserverNotification::<In, InError>::Error(error),
					);
				}
			},
			self.cancellation_id,
		);

		self.unsubscribe();
	}

	fn complete(&mut self) {
		let destination = self.destination;

		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if context.deferred_world.get_entity(destination).is_ok() {
					context.send_observer_notification(
						destination,
						ObserverNotification::<In, InError>::Complete,
					);
				}
			},
			self.cancellation_id,
		);

		self.unsubscribe();
	}
}

impl<In, InError> SubscriptionLike for EntityDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.teardown.unsubscribe();
		}
	}
}
