use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerHandle, SchedulerScheduleTaskExtension, SharedSubscriber, Signal,
	Subscriber, SubscriptionLike, TaskCancellationId, TaskResult,
};

struct FallbackWhenSilentSubscriberState<In> {
	next_observed_this_tick: Option<In>,
}

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	state: Arc<Mutex<FallbackWhenSilentSubscriberState<In>>>,
	scheduler_handle: SchedulerHandle<S>,
	cancellation_id: TaskCancellationId,
	_phantom_data: PhantomData<(In, InError, Fallback)>,
}

impl<In, InError, Fallback, Destination, S>
	FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Send + Sync,
	Destination: 'static + Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		fallback: Fallback,
		mut scheduler_handle: SchedulerHandle<S>,
	) -> Self {
		let shared_destination = SharedSubscriber::new(destination);
		let state = Arc::new(Mutex::new(FallbackWhenSilentSubscriberState {
			next_observed_this_tick: None,
		}));

		let scheduler_clone = scheduler_handle.clone();

		let mut scheduler = scheduler_handle.lock();
		let cancellation_id = scheduler.generate_cancellation_id();

		let shared_state_clone = state.clone();
		let mut shared_destination_clone = shared_destination.clone();
		scheduler.schedule_continuous_task(
			move |_tick, _context| {
				let observed_next = {
					let mut state = shared_state_clone.lock().unwrap_or_else(|a| a.into_inner());
					state.next_observed_this_tick.take()
				};

				let next = observed_next.unwrap_or_else(&(fallback));

				shared_destination_clone.next(next);

				TaskResult::Pending
			},
			cancellation_id,
		);

		Self {
			shared_destination,
			state,
			scheduler_handle: scheduler_clone,
			cancellation_id,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Fallback, Destination, S> Observer
	for FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let mut scheduler_state = self
			.state
			.lock()
			.unwrap_or_else(|poison_error| poison_error.into_inner());
		scheduler_state.next_observed_this_tick = Some(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.shared_destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.shared_destination.complete();
	}
}

impl<In, InError, Fallback, Destination, S> SubscriptionLike
	for FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler_handle.lock().cancel(self.cancellation_id);
		self.shared_destination.unsubscribe();
	}
}
