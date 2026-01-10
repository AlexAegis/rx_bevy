use rx_core_common::{
	RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionLike, TeardownCollectionExtension, WorkCancellationId, WorkResult,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::observable::IntervalObservableOptions;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct IntervalSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = usize>,
	S: Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<S>,
	cancellation_id: WorkCancellationId,
}

impl<Destination, S> IntervalSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = usize>,
	S: 'static + Scheduler,
{
	pub fn new(
		destination: Destination,
		interval_subscription_options: IntervalObservableOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let mut destination = SharedSubscriber::new(destination);

		if interval_subscription_options.start_on_subscribe {
			destination.next(0);
		}

		let cancellation_id = {
			let mut scheduler = scheduler.lock();
			let cancellation_id = scheduler.generate_cancellation_id();
			let destination_clone = destination.clone();

			let mut count = if interval_subscription_options.start_on_subscribe {
				1
			} else {
				0
			};

			scheduler.schedule_repeated_work(
				move |_, _| {
					let mut destination_lock = destination_clone.lock();

					if destination_lock.is_closed() {
						return WorkResult::Done;
					}

					destination_lock.next(count);
					count += 1;

					WorkResult::Pending
				},
				interval_subscription_options.duration,
				false,
				interval_subscription_options.max_emissions_per_tick,
				cancellation_id,
			);

			cancellation_id
		};

		let scheduler_clone = scheduler.clone();
		destination.add_fn(move || {
			scheduler_clone.lock().cancel(cancellation_id);
		});

		IntervalSubscription {
			destination,
			scheduler,
			cancellation_id,
		}
	}
}

impl<Destination, S> SubscriptionLike for IntervalSubscription<Destination, S>
where
	Destination: Subscriber<In = usize>,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler.lock().cancel(self.cancellation_id);
		if !self.destination.is_closed() {
			self.destination.unsubscribe();
		}
	}
}
