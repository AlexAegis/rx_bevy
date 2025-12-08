use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Scheduler, SchedulerHandle, Signal, Subscriber};

struct FallbackWhenSilentSubscriberState {
	next_was_observed_this_tick: bool,
}

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	#[destination]
	destination: Destination,
	state: Arc<Mutex<FallbackWhenSilentSubscriberState>>,

	fallback: Fallback,
	scheduler_handle: SchedulerHandle<S>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Fallback, Destination, S>
	FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		fallback: Fallback,
		scheduler_handle: SchedulerHandle<S>,
	) -> Self {
		Self {
			destination,
			state: Arc::new(Mutex::new(FallbackWhenSilentSubscriberState {
				next_was_observed_this_tick: false,
			})),
			fallback,
			scheduler_handle,
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
		let mut scheduler_state = self.state.lock().unwrap_or_else(|a| a.into_inner());
		scheduler_state.next_was_observed_this_tick = true;
		self.destination.next(next); // Should this happen here, or from the scheduled task?
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

/*
TODO: REimplement with the scheduler
impl<In, InError, Fallback, Destination> Tickable
	for FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	fn tick(&mut self, tick: Tick) {
		if !self.next_was_observed_this_tick {
			let fallback_value = (self.fallback)();
			self.next(fallback_value);
		} else {
			self.next_was_observed_this_tick = false;
		}

		self.destination.tick(tick);
	}
}
*/
