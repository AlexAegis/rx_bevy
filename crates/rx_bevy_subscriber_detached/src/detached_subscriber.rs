use rx_bevy_core::{
	Observer, ObserverInput, Operation, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Tick,
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
}

impl<'c, Destination> SubscriptionCollection<'c> for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection<'c>,
{
	#[inline]
	fn add<S: 'c + SubscriptionLike<Context = <Self as SignalContext>::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<Destination> Operation for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
