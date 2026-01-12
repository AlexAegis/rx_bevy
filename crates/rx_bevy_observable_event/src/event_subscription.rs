use bevy_ecs::{entity::Entity, event::Event, name::Name, observer::Observer};
use disqualified::ShortName;
use rx_bevy_common::{RxBevyScheduler, RxBevySchedulerDespawnEntityExtension};
use rx_core_common::{
	Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber, Subscriber,
	TeardownCollectionExtension,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::create_event_forwarder_observer_for_destination;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
}

impl<Destination> EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
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

			let cancellation_id = scheduler_lock.generate_cancellation_id();
			let despawn_invoke_id = scheduler_lock.generate_invoke_id();

			let scheduler_schedule_clone = scheduler.clone();
			scheduler_lock.schedule_immediate_work(
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

			(cancellation_id, despawn_invoke_id)
		};

		let scheduler_teardown_clone = scheduler.clone();
		shared_destination.add_fn(move || {
			let mut scheduler = scheduler_teardown_clone.lock();
			scheduler.invoke(despawn_invoke_id);
			scheduler.cancel(cancellation_id);
		});

		Self { shared_destination }
	}
}
