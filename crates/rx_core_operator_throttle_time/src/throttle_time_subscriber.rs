use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use rx_core_common::{
	LockWithPoisonBehavior, RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension,
	SharedSubscriber, Subscriber, SubscriberState, SubscriptionLike, Teardown, WorkCancellationId,
	WorkResult, WorkTick,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::ThrottleTimeOptions;

struct ThrottleTimeState<In> {
	last_value: Option<In>,
	last_tick_time: Duration,
	throttle_deadline: Option<Duration>,
	upstream_state: SubscriberState,
}

impl<In> ThrottleTimeState<In> {
	fn clear_throttle(&mut self) {
		self.throttle_deadline = None;
		self.last_value = None;
	}

	fn has_pending_trailing(&self, options: ThrottleTimeOptions) -> bool {
		options.output_behavior.emits_trailing() && self.last_value.is_some()
	}
}

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct ThrottleTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	options: ThrottleTimeOptions,
	scheduler: SchedulerHandle<S>,
	cancellation_id: WorkCancellationId,
	state: Arc<Mutex<ThrottleTimeState<Destination::In>>>,
}

impl<Destination, S> ThrottleTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(
		mut destination: Destination,
		options: ThrottleTimeOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();
		destination.add_teardown(Teardown::new_work_cancellation(
			cancellation_id,
			scheduler.clone(),
		));
		Self {
			destination: SharedSubscriber::new(destination),
			options,
			scheduler,
			cancellation_id,
			state: Arc::new(Mutex::new(ThrottleTimeState {
				last_value: None,
				last_tick_time: Duration::ZERO,
				throttle_deadline: None,
				upstream_state: SubscriberState::default(),
			})),
		}
	}

	fn schedule_throttle_work(&self) {
		let state = self.state.clone();
		let options = self.options;
		let mut destination = self.destination.clone();

		self.scheduler.lock().schedule_continuous_work(
			move |tick, _context| {
				let (continue_work, emit_value, should_complete, should_unsubscribe) = {
					let mut state = state.lock_ignore_poison();
					state.last_tick_time = tick.now();

					let deadline = state
						.throttle_deadline
						.expect("scheduled work always has an active throttle deadline");

					if state.last_tick_time < deadline {
						return WorkResult::Pending;
					}

					if deadline < state.last_tick_time {
						state.throttle_deadline = None;

						(
							false,
							None,
							state.upstream_state.is_completed(),
							state.upstream_state.is_closed_but_not_completed(),
						)
					} else {
						let emit_value = if options.output_behavior.emits_trailing() {
							state.last_value.take()
						} else {
							state.last_value = None;
							None
						};

						let continue_for_cooldown = emit_value.is_some();
						if !continue_for_cooldown {
							state.throttle_deadline = None;
						}

						(
							continue_for_cooldown,
							emit_value,
							state.upstream_state.is_completed(),
							state.upstream_state.is_closed_but_not_completed(),
						)
					}
				};

				if let Some(value) = emit_value {
					destination.next(value);
					if destination.is_closed() {
						return WorkResult::Done;
					}
				}

				if should_complete {
					destination.complete();
					return WorkResult::Done;
				}

				if should_unsubscribe {
					destination.unsubscribe();
					return WorkResult::Done;
				}

				if continue_work {
					WorkResult::Pending
				} else {
					WorkResult::Done
				}
			},
			self.cancellation_id,
		);
	}
}

impl<Destination, S> RxObserver for ThrottleTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let was_idle = {
			let mut state = self.state.lock_ignore_poison();
			let was_idle = state.throttle_deadline.is_none();
			match state.throttle_deadline {
				None => {
					// Idle
					state.throttle_deadline = Some(state.last_tick_time + self.options.duration);
					if self.options.output_behavior.emits_leading() {
						self.destination.next(next);
					} else if self.options.output_behavior.emits_trailing() {
						state.last_value = Some(next);
					};
				}
				Some(deadline) if deadline > state.last_tick_time => {
					// Throttling
					if self.options.output_behavior.emits_trailing() {
						state.last_value = Some(next);
					}
				}
				Some(_) => {
					// Throttle over, start new window
					state.throttle_deadline = Some(state.last_tick_time + self.options.duration);
					debug_assert!(
						self.options.output_behavior.emits_trailing(),
						"cooldown can only occur when trailing is enabled"
					);
					state.last_value = Some(next);
				}
			};
			was_idle
		};

		if was_idle {
			self.schedule_throttle_work();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		{
			let mut state = self.state.lock_ignore_poison();
			state.upstream_state.error();
			state.clear_throttle();
		}
		self.scheduler.lock().cancel(self.cancellation_id);
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		let should_complete_immediately = {
			let mut state = self.state.lock_ignore_poison();
			state.upstream_state.complete();

			if state.throttle_deadline.is_none() || !state.has_pending_trailing(self.options) {
				state.clear_throttle();
				true
			} else {
				false
			}
		};

		if should_complete_immediately {
			self.scheduler.lock().cancel(self.cancellation_id);
			self.destination.complete();
		}
	}
}

impl<Destination, S> SubscriptionLike for ThrottleTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		if self.destination.is_closed() {
			return true;
		}
		let state = self.state.lock_ignore_poison();
		state.upstream_state.is_closed()
	}

	fn unsubscribe(&mut self) {
		if self.is_closed() {
			return;
		}

		let should_unsubscribe_immediately = {
			let mut state = self.state.lock_ignore_poison();
			state.upstream_state.unsubscribe();

			if state.throttle_deadline.is_none() || !state.has_pending_trailing(self.options) {
				state.clear_throttle();
				true
			} else {
				false
			}
		};

		if should_unsubscribe_immediately {
			self.scheduler.lock().cancel(self.cancellation_id);
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, S> Drop for ThrottleTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
