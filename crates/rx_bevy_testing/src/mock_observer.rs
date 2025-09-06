use std::marker::PhantomData;

use rx_bevy_core::{Observer, ObserverInput, SubscriptionLike, Tick};

#[derive(Debug)]
pub struct MockObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	pub closed: bool,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ObserverInput for MockObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

pub struct MockContext<In, InError>
where
	In: 'static,
	InError: 'static,
{
	pub values: Vec<In>,
	pub errors: Vec<InError>,
	pub ticks: Vec<Tick>,
	pub completed: usize,
	pub values_after_closed: Vec<In>,
	pub errors_after_closed: Vec<InError>,
	pub ticks_after_closed: Vec<Tick>,
	pub completed_after_closed: usize,
}

impl<In, InError> MockContext<In, InError>
where
	In: 'static,
	InError: 'static,
{
	pub fn nothing_happened_after_closed(&self) -> bool {
		self.values_after_closed.is_empty()
			&& self.errors_after_closed.is_empty()
			&& self.completed_after_closed == 0
			&& self.ticks_after_closed.is_empty()
	}
}

impl<In, InError> Default for MockContext<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn default() -> Self {
		Self {
			values: Vec::with_capacity(1),
			errors: Vec::with_capacity(1),
			ticks: Vec::with_capacity(1),
			values_after_closed: Vec::new(),
			errors_after_closed: Vec::new(),
			ticks_after_closed: Vec::new(),
			completed: 0,
			completed_after_closed: 0,
		}
	}
}

impl<In, InError> Observer for MockObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Context = MockContext<In, InError>;

	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			context.values.push(next);
		} else {
			context.values_after_closed.push(next);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			context.errors.push(error);
		} else {
			context.errors_after_closed.push(error);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			context.completed += 1;
			self.unsubscribe(context);
		} else {
			context.completed_after_closed += 1;
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			context.ticks.push(tick);
		} else {
			context.ticks_after_closed.push(tick);
		}
	}
}

impl<In, InError> SubscriptionLike<MockContext<In, InError>> for MockObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut MockContext<In, InError>) {
		self.closed = true;
	}
}

impl<In, InError> Default for MockObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
