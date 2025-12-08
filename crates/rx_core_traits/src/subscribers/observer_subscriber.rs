use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SubscriptionLike,
	Teardown, TeardownCollection, WithPrimaryCategory,
};

use crate::SubscriptionData;

/// This subscriber acts as the subscriptions boundary by not forwarding
/// `unsubscribe` calls downstream.
#[derive(Debug)]

pub struct ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	destination: Destination,
	teardown: SubscriptionData,
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

impl<Destination> WithPrimaryCategory for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for ObserverSubscriber<Destination> where
	Destination: Observer
{
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

impl<Destination> ObserverInput for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Drop for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.unsubscribe();
		}
	}
}
