use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_common::{LockWithPoisonBehavior, RxObserver, SharedSubscriber, Signal, Subscriber};
use rx_core_macro_observer_derive::RxObserver;

#[derive_where(Default)]
pub struct WithLatestFromInnerDestinationState<In>
where
	In: Signal,
{
	latest_in: Option<In>,
	error: bool,
	completed: bool,
}

impl<In> WithLatestFromInnerDestinationState<In>
where
	In: Signal,
{
	#[inline]
	pub fn is_finished(&self) -> bool {
		self.completed || self.error
	}

	pub fn get_latest_value(&self) -> &Option<In> {
		&self.latest_in
	}

	fn next(&mut self, next: In) {
		if !self.is_finished() {
			self.latest_in = Some(next);
		}
	}

	fn error(&mut self) {
		if !self.is_finished() {
			self.error = true;
		}
	}

	fn complete(&mut self) {
		if !self.is_finished() {
			self.completed = true;
		}
	}
}

#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(ErrorDestination::InError)]
pub struct WithLatestFromInnerDestination<In, ErrorDestination>
where
	In: Signal,
	ErrorDestination: 'static + Subscriber,
{
	state: Arc<Mutex<WithLatestFromInnerDestinationState<In>>>,
	error_destination: SharedSubscriber<ErrorDestination>,
}

impl<In, ErrorDestination> WithLatestFromInnerDestination<In, ErrorDestination>
where
	In: Signal,
	ErrorDestination: Subscriber,
{
	pub fn new(error_destination: SharedSubscriber<ErrorDestination>) -> Self {
		Self {
			state: Arc::new(Mutex::new(
				WithLatestFromInnerDestinationState::<In>::default(),
			)),
			error_destination,
		}
	}

	#[inline]
	pub fn get_state(&self) -> Arc<Mutex<WithLatestFromInnerDestinationState<In>>> {
		self.state.clone()
	}
}

impl<In, ErrorDestination> RxObserver for WithLatestFromInnerDestination<In, ErrorDestination>
where
	In: Signal,
	ErrorDestination: 'static + Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.state.lock_ignore_poison().next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.state.lock_ignore_poison().error();
		self.error_destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.state.lock_ignore_poison().complete();
		// Would make it impossible to emit further values
		if self.state.lock_ignore_poison().get_latest_value().is_none() {
			self.error_destination.complete();
		}
	}
}
