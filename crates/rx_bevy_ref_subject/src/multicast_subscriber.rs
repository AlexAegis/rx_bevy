use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Tick,
};

use crate::MulticastDestination;

pub struct MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) key: usize,
	pub(crate) destination: Destination,
	pub(crate) subscriber_ref: Arc<
		RwLock<
			MulticastDestination<
				Destination::In,
				Destination::InError,
				<Self as SignalContext>::Context,
			>,
		>,
	>,
}

impl<Destination> SignalContext for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
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

impl<Destination> SubscriptionLike for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		// See the subjects Teardown Fn to learn how this subscriber is
		// removed from the subject.
		self.destination.unsubscribe(context);
	}

	fn is_closed(&self) -> bool {
		if let Ok(subject) = self.subscriber_ref.read() {
			subject
				.slab
				.get(self.key)
				.map(|destination| destination.is_closed())
				.unwrap_or(!subject.slab.contains(self.key))
		} else {
			self.destination.is_closed()
		}
	}
}

impl<Destination> SubscriptionCollection for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber + SubscriptionCollection,
{
	#[inline]
	fn add<S: 'static + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<Destination> ObserverInput for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Operation for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
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

impl<Destination> Drop for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.assert_closed_when_dropped();
	}
}
