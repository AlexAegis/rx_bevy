use rx_bevy_core::{
	ArcSubscriber, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick, Tickable,
	WithContext,
};

use crate::InnerRcSubscriber;

/// Acquired by calling `downgrade` on `RcSubscriber`
pub struct WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	// TODO: Since in bevy this won't be a pointer just an Entity, maybe we'd need a enum or trait here
	pub(crate) destination: ArcSubscriber<InnerRcSubscriber<Destination>>,
	pub(crate) closed: bool,
}

impl<Destination> WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&InnerRcSubscriber<Destination>),
	{
		self.destination.read(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn write<F>(&mut self, writer: F)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>),
	{
		self.destination.write(writer);
	}
}

impl<Destination> Clone for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
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
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithContext for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
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
}

impl<Destination> Tickable for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
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
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Destination> Drop for WeakRcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.destination.write(|destination| {
				let mut context = destination.get_context_to_unsubscribe_on_drop();
				destination.complete_if_can(&mut context);
				destination.unsubscribe_if_can(&mut context);
			});
		}
	}
}
