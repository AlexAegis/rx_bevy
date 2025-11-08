use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber,
	SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
	WithPrimaryCategory, WithSubscriptionContext,
};

use crate::SubscriptionData;

/// This subscriber acts as the subscriptions boundary by not forwarding
/// `unsubscribe` calls downstream.
pub struct DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	destination: Destination,
	teardown: SubscriptionData<Destination::Context>,
}

impl<Destination> DetachedSubscriber<Destination>
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

impl<Destination> WithPrimaryCategory for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for DetachedSubscriber<Destination> where
	Destination: Observer
{
}

impl<Destination> WithSubscriptionContext for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.next(next, context);
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.error(error, context);
		}
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.complete(context);
		}
	}
}

impl<Destination> Tickable for DetachedSubscriber<Destination>
where
	Destination: Observer,
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

impl<Destination> SubscriptionLike for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.teardown.unsubscribe(context);
	}
}

impl<Destination> TeardownCollection for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}

impl<Destination> ObserverInput for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Drop for DetachedSubscriber<Destination>
where
	Destination: Observer,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			println!("DROPPING UNCLOSED DETACHED SUB!!!!!");
			let mut context = Destination::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
