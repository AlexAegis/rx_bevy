use rx_bevy_observable::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

use crate::MulticastOuterSubscriber;

pub struct MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub(crate) destination: Destination,
	pub(crate) outer: MulticastOuterSubscriber<Destination>,
}

impl<Destination> Observer for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<Destination> SubscriptionLike for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn unsubscribe(&mut self) {
		println!("MulticastInnerSubscriber unsubscribe 1");

		self.outer.unsubscribe();
	}

	fn is_closed(&self) -> bool {
		self.outer.is_closed()
	}
}

impl<Destination> ObserverInput for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Operation for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Destination = Destination;
}

impl<Destination> Drop for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
