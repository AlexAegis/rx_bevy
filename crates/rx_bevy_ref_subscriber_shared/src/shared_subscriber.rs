use rx_bevy_core::{
	Observer, ObserverInput, Operation, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

/// It must hold the invariant that the cloned destination points to the
/// exact same thing. Like an `Arc` or an `Entity`
pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	destination: Destination,
}

impl<Destination> From<Destination> for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Destination),
	{
		reader(&self.destination)
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut Destination),
	{
		reader(&mut self.destination)
	}
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
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
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}
}

impl<Destination> SubscriptionCollection for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	/// Should not unsubscribe on drop as it's shared
	fn drop(&mut self) {}
}

impl<Destination> Operation for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
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
