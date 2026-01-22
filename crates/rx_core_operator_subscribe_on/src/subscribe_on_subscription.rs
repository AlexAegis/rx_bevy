use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use core::marker::PhantomData;

use rx_core_common::{
	LockWithPoisonBehavior, Observable, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension,
	SharedSubscriber, SharedSubscription, Subscriber, SubscriptionLike, Teardown,
	TeardownCollectionExtension,
};
use rx_core_macro_subscription_derive::RxSubscription;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct SubscribeOnSubscription<Destination, Source>
where
	Destination: 'static + Subscriber,
	Source:
		'static + Observable<Out = Destination::In, OutError = Destination::InError> + Send + Sync,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	_phantom_source: PhantomData<Source>,
}

impl<Destination, Source> SubscribeOnSubscription<Destination, Source>
where
	Destination: 'static + Subscriber,
	Source:
		'static + Observable<Out = Destination::In, OutError = Destination::InError> + Send + Sync,
{
	pub fn new<S: 'static + Scheduler + Send + Sync>(
		destination: Destination,
		source: Arc<Mutex<Source>>,
		delay: Duration,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let mut destination = SharedSubscriber::new(destination);
		let mut upstream_subscription = SharedSubscription::default();

		let cancellation_id = {
			let mut scheduler = scheduler.lock();
			let cancellation_id = scheduler.generate_cancellation_id();

			let destination_clone = destination.clone();
			let mut upstream_subscription_clone = upstream_subscription.clone();
			let source_clone = source.clone();

			scheduler.schedule_delayed_work(
				move |_, _| {
					if destination_clone.is_closed() {
						return;
					}

					let subscription = source_clone
						.lock_ignore_poison()
						.subscribe(destination_clone.clone());

					if destination_clone.is_closed() {
						let mut subscription = subscription;
						subscription.unsubscribe();
						return;
					}

					upstream_subscription_clone.add(subscription);
				},
				delay,
				cancellation_id,
			);

			cancellation_id
		};

		upstream_subscription.add(Teardown::new_work_cancellation(cancellation_id, scheduler));
		destination.add(upstream_subscription.clone());

		Self {
			destination,
			_phantom_source: PhantomData,
		}
	}
}
