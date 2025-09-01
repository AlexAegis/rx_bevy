use std::sync::{Arc, RwLock};

use rx_bevy_core::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

use crate::MulticastDestination;

pub struct MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) key: usize,
	pub(crate) destination: Destination,
	pub(crate) subscriber_ref:
		Arc<RwLock<MulticastDestination<Destination::In, Destination::InError>>>,
}

impl<Destination> Observer for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		self.destination.tick(tick);
	}
}

impl<Destination> SubscriptionLike for MulticastSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn unsubscribe(&mut self) {
		// See the subjects Teardown Fn to learn how this subscriber is
		// removed from the subject.
		self.destination.unsubscribe();
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

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
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
		self.unsubscribe();
	}
}
