use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, Subscriber, SubscriptionLike, Teardown, Tick,
};

/// A helper subscriber that does not forward completion and unsubscribe signals.
/// Creating a barrier for these lifecycle signals.
/// Should only be used internally inside other subscribers, and they should
/// guarantee managing the destination completion and unsubscription.
pub struct DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
}

impl<Destination> DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> ObserverInput for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, _context: &mut Self::Context) {
		// Disconnected on purpose
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		// The subscription management is handled by the implementor
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}
