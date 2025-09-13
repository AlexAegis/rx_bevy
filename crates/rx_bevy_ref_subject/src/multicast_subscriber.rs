use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Tick,
};

use crate::MulticastDestination;

pub struct MulticastSubscriber<'c, Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) key: usize,
	pub(crate) destination: Destination,
	pub(crate) subscriber_ref: Arc<
		RwLock<
			MulticastDestination<
				'c,
				Destination::In,
				Destination::InError,
				<Self as SignalContext>::Context,
			>,
		>,
	>,
}

impl<'c, Destination> SignalContext for MulticastSubscriber<'c, Destination>
where
	Destination: 'static + Subscriber,
{
	type Context = Destination::Context;
}

impl<'c, Destination> Observer for MulticastSubscriber<'c, Destination>
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

impl<'c, Destination> SubscriptionLike for MulticastSubscriber<'c, Destination>
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

impl<'c, Destination> SubscriptionCollection<'c> for MulticastSubscriber<'c, Destination>
where
	Destination: 'static + Subscriber + SubscriptionCollection<'c>,
{
	#[inline]
	fn add<S: 'static + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<'c, Destination> ObserverInput for MulticastSubscriber<'c, Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<'c, Destination> Operation for MulticastSubscriber<'c, Destination>
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

impl<'c, Destination> Drop for MulticastSubscriber<'c, Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.assert_closed_when_dropped();
	}
}
