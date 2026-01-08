use bevy_ecs::entity::Entity;
use rx_core_common::*;
use rx_core_macro_subscription_derive::RxSubscription;

use rx_bevy_common::{
	CommandSubscribeExtension, RxBevyScheduler, RxBevySchedulerDespawnEntityExtension,
};

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct ProxySubscription<Destination>
where
	Destination: 'static + Subscriber,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	closed_flag: SubscriptionClosedFlag,
	despawn_invoke_id: WorkInvokeId,
	cancellation_id: WorkCancellationId,
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
		let shared_destination = SharedSubscriber::new(destination);

		let shared_destination_clone = shared_destination.clone();

		let scheduler_subscription_clone = scheduler.clone();
		let scheduler_schedule_clone = scheduler.clone();
		let mut scheduler = scheduler.lock();
		let cancellation_id = scheduler.generate_cancellation_id();
		let despawn_invoke_id = scheduler.generate_invoke_id();

		scheduler.schedule_immediate_work(
			move |_, context| {
				let proxy_subscription_entity = context
					.deferred_world
					.commands()
					.subscribe::<_>(target_observable_entity, shared_destination_clone);

				scheduler_schedule_clone
					.lock()
					.schedule_invoked_despawn_entity(proxy_subscription_entity, despawn_invoke_id);
			},
			cancellation_id,
		);

		Self {
			despawn_invoke_id,
			cancellation_id,
			scheduler: scheduler_subscription_clone,
			destination: shared_destination,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber,
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
			scheduler.cancel(self.cancellation_id);
		}
	}
}
