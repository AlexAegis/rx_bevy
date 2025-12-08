use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Subscriber,
	SubscriptionLike, Teardown, TeardownCollection, WithPrimaryCategory,
};

/// A wrapper around a subscriber, that simply forwards everything except ticks.
pub struct UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
}

impl<Destination> UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> WithPrimaryCategory for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for UnscheduledSubscriber<Destination> where
	Destination: Subscriber
{
}

impl<Destination> Observer for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
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
}

impl<Destination> SubscriptionLike for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<Destination> TeardownCollection for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.destination.add_teardown(teardown);
	}
}

impl<Destination> ObserverInput for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}
