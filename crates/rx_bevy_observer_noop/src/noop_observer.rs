use std::marker::PhantomData;

use rx_bevy_core::{Observer, ObserverInput, SignalContext, SubscriptionLike, UpgradeableObserver};
use rx_bevy_subscriber_observer::ObserverSubscriber;

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
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for NoopObserver<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::SubscriptionContext) {
		self.closed = true;
	}
}

impl<In, InError> Default for NoopObserver<In, InError> {
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
