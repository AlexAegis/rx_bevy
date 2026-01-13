use bevy_ecs::entity::Entity;
use rx_core_common::*;
use rx_core_macro_subscription_derive::RxSubscription;

use rx_bevy_common::{
	CommandSubscribeExtension, RxBevyScheduler, RxBevySchedulerDespawnEntityExtension,
};

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct ProxySubscription<Destination>
where
	Destination: 'static + Subscriber,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
}

impl<Destination> ProxySubscription<Destination>
where
	Destination: 'static + Subscriber + UpgradeableObserver,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	pub fn new(
		target_observable_entity: Entity,
		destination: Destination,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let mut shared_destination = SharedSubscriber::new(destination);

		let (cancellation_id, despawn_invoke_id) = {
			let scheduler_schedule_clone = scheduler.clone();
			let shared_destination_clone = shared_destination.clone();
			let mut scheduler = scheduler.lock();
			let cancellation_id = scheduler.generate_cancellation_id();
			let despawn_invoke_id = scheduler.generate_invoke_id();

			scheduler.schedule_immediate_work(
				move |_, context| {
					let proxy_subscription_entity = context
						.deferred_world
						.commands()
						.subscribe(target_observable_entity, shared_destination_clone);

					scheduler_schedule_clone
						.lock()
						.schedule_invoked_despawn_entity(
							proxy_subscription_entity,
							despawn_invoke_id,
						);
				},
				cancellation_id,
			);

			(cancellation_id, despawn_invoke_id)
		};

		shared_destination.add(Teardown::new_work_invokation_and_cancellation(
			despawn_invoke_id,
			cancellation_id,
			scheduler,
		));

		Self { shared_destination }
	}
}
