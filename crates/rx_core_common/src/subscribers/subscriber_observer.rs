use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{Observer, SubscriptionLike, Teardown, TeardownCollection};

use crate::SubscriptionData;

/// This subscriber acts as the subscriptions boundary by not forwarding
/// `unsubscribe` calls downstream.
#[derive(RxSubscriber, Debug)]
#[_rx_core_common_crate(crate)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
pub struct ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	#[teardown]
	teardown: SubscriptionData,
	#[destination]
	destination: Destination,
}

impl<Destination> ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Destination> Observer for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.destination.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
		}
	}
}

impl<Destination> SubscriptionLike for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.teardown.unsubscribe();
	}
}

impl<Destination> TeardownCollection for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.teardown.add_teardown(teardown);
	}
}
