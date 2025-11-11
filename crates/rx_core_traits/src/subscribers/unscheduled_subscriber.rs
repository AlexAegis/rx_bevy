use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Subscriber,
	SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
	WithPrimaryCategory, WithSubscriptionContext,
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

impl<Destination> WithSubscriptionContext for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<Destination> Tickable for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn tick(
		&mut self,
		_tick: Tick,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// Does not do anything on tick as the destination is not (necessarily)
		// tickable!
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
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}
}

impl<Destination> TeardownCollection for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<Destination> ObserverInput for UnscheduledSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}
