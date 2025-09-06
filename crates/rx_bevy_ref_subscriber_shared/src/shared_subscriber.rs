use rx_bevy_core::{
	ChannelContext, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
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

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Subscriber + Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut ChannelContext) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut ChannelContext) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut ChannelContext) {
		self.destination.complete(context);
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick, context: &mut ChannelContext) {
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
	fn unsubscribe(&mut self, context: &mut ChannelContext) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add(&mut self, subscription: impl Into<Teardown>, context: &mut ChannelContext) {
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
