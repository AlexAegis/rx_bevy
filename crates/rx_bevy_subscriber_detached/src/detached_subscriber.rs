use rx_bevy_core::{
	Observer, ObserverInput, Operation, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
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
	type Context<'c> = Destination::Context<'c>;
}

impl<Destination> Observer for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete<'c>(&mut self, _context: &mut Self::Context<'c>) {
		// Disconnected on purpose
	}

	#[inline]
	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
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
	fn unsubscribe<'c>(&mut self, _context: &mut Self::Context<'c>) {
		// The subscription management is handled by the implementor
	}
}

impl<Destination> SubscriptionCollection for DetachedSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
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
