use std::sync::{Arc, RwLock};

use rx_bevy_observable::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

use crate::MulticastDestination;

pub struct MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) key: usize,
	pub(crate) subscriber_ref:
		Arc<RwLock<MulticastDestination<Destination::In, Destination::InError>>>,
}

impl<Destination> Observer for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.slab.get_mut(self.key) {
				sub.next(next);
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.slab.get_mut(self.key) {
				sub.error(error);
			}
		}
	}

	fn complete(&mut self) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.slab.get_mut(self.key) {
				sub.complete();
			}
		}
	}
}

impl<Destination> SubscriptionLike for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn unsubscribe(&mut self) {
		println!("MulticastOuterSubscriber unsubscribe 1");

		let self_ref = self
			.subscriber_ref
			.write()
			.map(|mut d| d.take(self.key))
			.expect("no poison");

		println!("    MulticastOuterSubscriber unsubscribes2");

		if let Some(mut self_ref) = self_ref {
			println!("    MulticastOuterSubscriber unsubscribe 3");

			self_ref.unsubscribe();
		}
	}

	fn is_closed(&self) -> bool {
		if let Ok(subject) = self.subscriber_ref.read() {
			subject
				.slab
				.get(self.key)
				.map(|destination| destination.is_closed())
				.unwrap_or(!subject.slab.contains(self.key))
		} else {
			true
		}
	}
}

impl<Destination> ObserverInput for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Operation for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Destination = Destination;
}

impl<Destination> Drop for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
