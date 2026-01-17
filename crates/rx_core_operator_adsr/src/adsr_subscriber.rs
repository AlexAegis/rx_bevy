use core::marker::PhantomData;
use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use rx_core_common::{
	PhantomInvariant, RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension,
	SharedSubscriber, Signal, Subscriber, SubscriptionLike, WorkCancellationId, WorkResult,
	WorkTick,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{
	AdsrEnvelopePhase, AdsrEnvelopeState, AdsrSignal, AdsrTrigger, operator::AdsrOperatorOptions,
};

#[derive(Debug)]
struct AdsrEnvelopeSharedState {
	is_getting_activated: bool,
	last_signal_was_none: bool,
	options: AdsrOperatorOptions,
}

#[derive(RxSubscriber, Debug)]
#[rx_in(AdsrTrigger)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
pub struct AdsrSubscriber<InError, Destination, S>
where
	InError: Signal,
	Destination: 'static + Subscriber<In = AdsrSignal, InError = InError>,
	S: Scheduler,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	shared_state: Arc<Mutex<AdsrEnvelopeSharedState>>,
	scheduler: SchedulerHandle<S>,
	cancellation_id: WorkCancellationId,
	_phantom_data: PhantomInvariant<InError>,
}

impl<InError, Destination, S> AdsrSubscriber<InError, Destination, S>
where
	InError: Signal,
	Destination: 'static + Subscriber<In = AdsrSignal, InError = InError>,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let shared_destination = SharedSubscriber::new(destination);
		let shared_state = Arc::new(Mutex::new(AdsrEnvelopeSharedState {
			is_getting_activated: false,
			last_signal_was_none: false,
			options,
		}));
		let shared_state_clone = shared_state.clone();
		let shared_destination_clone = shared_destination.clone();
		let scheduler_clone = scheduler.clone();
		let mut scheduler_lock = scheduler_clone.lock();
		let cancellation_id = scheduler_lock.generate_cancellation_id();
		let mut last_now = Duration::from_millis(0);
		let mut envelope_state = AdsrEnvelopeState::default();
		scheduler_lock.schedule_continuous_work(
			move |tick, _context| {
				let mut destination_lock = shared_destination_clone.lock();
				if destination_lock.is_closed() {
					return WorkResult::Done;
				}

				let next = {
					let Ok(mut state) = shared_state_clone.lock() else {
						return WorkResult::Done;
					};

					let now = tick.now();
					let delta = now - last_now;
					last_now = now;

					let next = envelope_state.calculate_output(
						state.options.envelope,
						state.is_getting_activated,
						now,
						delta,
					);

					if state.options.reset_input_on_tick {
						state.is_getting_activated = false;
					}

					let current_phase_is_none =
						matches!(next.adsr_envelope_phase, AdsrEnvelopePhase::None);
					let last_signal_was_none = state.last_signal_was_none;
					state.last_signal_was_none = current_phase_is_none;
					// If `always_emit_none`, it always emits.
					// If the current phase isn't `None`, then it also should emit because it's a useful value.
					// If the last signal was not `None`, then the current value should be emitted even if it's
					// a `None` to have at least one `None` emitted at the end of an activation.
					if state.options.always_emit_none
						|| !current_phase_is_none
						|| !last_signal_was_none
					{
						Some(next)
					} else {
						None
					}
				};

				if let Some(next) = next {
					destination_lock.next(next);
					if destination_lock.is_closed() {
						return WorkResult::Done;
					}
				}

				WorkResult::Pending
			},
			cancellation_id,
		);

		Self {
			shared_destination,
			scheduler,
			shared_state,
			cancellation_id,
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, Destination, S> RxObserver for AdsrSubscriber<InError, Destination, S>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	S: Scheduler,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let mut state = self.shared_state.lock().unwrap_or_else(|p| p.into_inner());
		state.is_getting_activated = next.activated;

		if let Some(envelope_change) = next.envelope_changes {
			state.options.envelope.apply_change(envelope_change);
		}
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

impl<InError, Destination, S> SubscriptionLike for AdsrSubscriber<InError, Destination, S>
where
	InError: Signal,
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
	S: Scheduler,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler.lock().cancel(self.cancellation_id);
		self.shared_destination.unsubscribe();
	}
}
