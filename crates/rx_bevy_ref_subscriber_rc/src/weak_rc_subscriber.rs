use rx_bevy_core::{
	ArcSubscriber, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

use crate::RcDestination;

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO: Since in bevy this won't be a pointer just an Entity, maybe we'd need a enum or trait here
	pub(crate) destination: ArcSubscriber<RcDestination<Destination>>,
	pub(crate) closed: bool,
}

impl<Destination> WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&RcDestination<Destination>),
	{
		self.destination.read(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn write<F>(&mut self, writer: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		self.destination.write(writer);
	}
}

impl<Destination> Clone for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			closed: self.closed,
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SignalContext for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.closed = true;
			self.destination.unsubscribe(context);
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<Destination> SubscriptionCollection for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
	Destination: SubscriptionCollection,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<Destination> Drop for WeakRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.destination.write(|destination| {
				let mut context = destination.get_unsubscribe_context();
				destination.complete_if_can(&mut context);
				destination.unsubscribe_if_can(&mut context);
			});
		}
	}
}
