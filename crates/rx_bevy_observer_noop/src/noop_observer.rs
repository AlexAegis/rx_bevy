use std::marker::PhantomData;

use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, SubscriptionLike, Teardown, Tickable, WithContext,
};

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
	Context: SignalContext,
{
	fn next(&mut self, _next: Self::In, _context: &mut Self::Context) {}

	fn error(&mut self, _error: Self::InError, _context: &mut Self::Context) {
		#[cfg(feature = "panic_on_error")]
		{
			panic!("noop observer observed an error!")
		}
	}

	fn complete(&mut self, _context: &mut Self::Context) {}
}

impl<In, InError, Context> Tickable for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	fn tick(&mut self, _tick: rx_bevy_core::Tick, _context: &mut Self::Context) {}
}

impl<In, InError, Context> WithContext for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: SignalContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		self.closed = true;
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		teardown.execute(context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
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
