use crate::{
	Observer, ObserverInput, Operation, ShareableSubscriber, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

pub struct SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	destination: Sharer::Shared,
}

impl<Destination, Sharer> From<Destination> for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination, Sharer> SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Sharer::share(destination),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Sharer::Shared),
	{
		reader(&self.destination)
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut Sharer::Shared),
	{
		reader(&mut self.destination)
	}
}

impl<Destination, Sharer> Clone for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination, Sharer> ObserverInput for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination, Sharer> SignalContext for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	type Context = Destination::Context;
}

impl<Destination, Sharer> Observer for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
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

impl<Destination, Sharer> SubscriptionLike for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination, Sharer> SubscriptionCollection for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
	Sharer::Shared: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<Destination, Sharer> Drop for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
	}
}

impl<Destination, Sharer> Operation for SharedSubscriber<Destination, Sharer>
where
	Destination: 'static + Subscriber,
	Sharer: ShareableSubscriber<Destination>,
{
	type Destination = Destination;
}
