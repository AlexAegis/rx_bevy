use std::marker::PhantomData;

use rx_bevy_core::{DropContext, Observer, ObserverInput, SignalContext, SubscriptionLike};

#[derive(Debug)]
pub struct NoopObserver<In, InError, Context> {
	closed: bool,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> ObserverInput for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	fn next(&mut self, _next: Self::In, _context: &mut Self::Context) {}

	fn error(&mut self, _error: Self::InError, _context: &mut Self::Context) {
		#[cfg(feature = "panic_on_error")]
		{
			panic!("noop observer observed an error!")
		}
	}

	fn complete(&mut self, _context: &mut Self::Context) {}

	fn tick(&mut self, _tick: rx_bevy_core::Tick, _context: &mut Self::Context) {}
}

impl<In, InError, Context> SignalContext for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		self.closed = true;
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<In, InError, Context> Default for NoopObserver<In, InError, Context> {
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
