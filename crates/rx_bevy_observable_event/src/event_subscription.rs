use bevy_ecs::{entity::Entity, event::EntityEvent, name::Name, observer::Observer};
use disqualified::ShortName;
use rx_bevy_common::{
	RxBevyScheduler, RxBevySchedulerDespawnEntityExtension, SubscriptionSatellite,
};
use rx_core_common::{
	RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, Teardown, TeardownCollectionExtension,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::create_event_forwarder_observer_for_destination;

#[derive(RxSubscription)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: EntityEvent + Clone,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
}

impl<Destination> EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: EntityEvent + Clone,
{
	pub fn new(
		observed_event_source_entity: Entity,
		destination: Destination,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let mut shared_destination = SharedSubscriber::new(destination);

		let (cancellation_id, despawn_invoke_id) = {
			let mut scheduler_lock = scheduler.lock();
			let shared_destination_clone = shared_destination.clone();
			let mut shared_destination_despawn_clone = shared_destination.clone();

			let cancellation_id = scheduler_lock.generate_cancellation_id();
			let despawn_event_observer_invoke_id = scheduler_lock.generate_invoke_id();

			let scheduler_schedule_clone = scheduler.clone();
			scheduler_lock.schedule_immediate_work(
				move |_, context| {
					let mut commands = context.deferred_world.commands();
					let observer_satellite_entity = commands.spawn((
						Name::new(format!("Event Observer of {}", ShortName::of::<Self>())),
						Observer::new(create_event_forwarder_observer_for_destination(
							shared_destination_clone,
						))
						.with_entity(observed_event_source_entity)
						.with_error_handler(bevy_ecs::error::error),
						SubscriptionSatellite::new_with_teardown(
							observed_event_source_entity,
							Teardown::new(move || {
								shared_destination_despawn_clone.complete();
							}),
						),
					));

					scheduler_schedule_clone
						.lock()
						.schedule_invoked_despawn_entity(
							observer_satellite_entity.id(),
							despawn_event_observer_invoke_id,
						);
				},
				cancellation_id,
			);

			(cancellation_id, despawn_event_observer_invoke_id)
		};

		shared_destination.add(Teardown::new_work_invokation_and_cancellation(
			despawn_invoke_id,
			cancellation_id,
			scheduler,
		));

		Self { shared_destination }
	}
}
