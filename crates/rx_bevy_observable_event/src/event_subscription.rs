use bevy_ecs::{entity::Entity, event::Event, name::Name, observer::Observer};
use disqualified::ShortName;
use rx_bevy_context::{RxBevyScheduler, RxBevySchedulerDespawnEntityExtension};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Scheduler, SchedulerHandle, SchedulerScheduleTaskExtension, SharedSubscriber, Subscriber,
	SubscriptionClosedFlag, SubscriptionLike, TaskInvokeId, Teardown, TeardownCollection,
};

use crate::create_event_forwarder_observer_for_destination;

#[derive(RxSubscription)]
pub struct EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	despawn_invoke_id: TaskInvokeId,
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	pub fn new(
		observed_event_source_entity: Entity,
		destination: Destination,
		mut scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let shared_destination = SharedSubscriber::new(destination);

		let scheduler_subscription_clone = scheduler.clone();
		let mut scheduler_schedule_clone = scheduler.clone();
		let mut scheduler_lock = scheduler.lock();
		let shared_destination_clone = shared_destination.clone();

		let cancellation_id = scheduler_lock.generate_cancellation_id();
		let despawn_invoke_id = scheduler_lock.generate_invoke_id();

		scheduler_lock.schedule_immediate_task(
			move |_, context| {
				let mut commands = context.deferred_world.commands();
				let observer_satellite_entity = commands.spawn((
					Name::new(format!("Event Observer of {}", ShortName::of::<Self>())),
					Observer::new(create_event_forwarder_observer_for_destination(
						shared_destination_clone,
					))
					.with_entity(observed_event_source_entity),
				));

				scheduler_schedule_clone
					.lock()
					.schedule_invoked_despawn_entity(
						observer_satellite_entity.id(),
						despawn_invoke_id,
					);
			},
			cancellation_id,
		);

		Self {
			despawn_invoke_id,
			destination: shared_destination,
			scheduler: scheduler_subscription_clone,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe();
			let mut scheduler = self.scheduler.lock();
			scheduler.invoke(self.despawn_invoke_id);
		}
	}
}

impl<Destination> TeardownCollection for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}
