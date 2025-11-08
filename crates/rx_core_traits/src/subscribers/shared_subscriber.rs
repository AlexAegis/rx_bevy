use core::marker::PhantomData;

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Subscriber,
	SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable, WithPrimaryCategory,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::{DestinationAllocator, DestinationSharedTypes, SharedDestination},
	},
};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
// TODO: Maybe this and RcSubscriber should be joined together
pub struct SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	shared_destination: <Destination as DestinationSharedTypes>::Shared,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	pub fn new(
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			shared_destination: <Destination as DestinationSharedTypes>::Sharer::share(
				destination,
				context,
			),
			_phantom_data: PhantomData,
		}
	}

	pub fn clone_with_context(
		&mut self,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			shared_destination: self.shared_destination.clone_with_context(context),
			_phantom_data: PhantomData,
		}
	}

	pub fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Destination, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	{
		self.shared_destination
			.access_with_context(accessor, context);
	}

	pub fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(
			&mut Destination,
			&mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
		),
	{
		self.shared_destination
			.access_with_context_mut(accessor, context);
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Context = Destination::Context;
}

impl<Destination> WithPrimaryCategory for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for SharedSubscriber<Destination> where
	Destination: 'static + Subscriber + Send + Sync
{
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.shared_destination.complete(context);
	}
}

impl<Destination> Tickable for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.access_with_context_mut(
			move |destination: &mut Destination,
			      context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>| {
				destination.tick(tick.clone(), context)
			},
			context,
		);
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.shared_destination.unsubscribe(context);
	}
}

impl<Destination> TeardownCollection for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.add_teardown(teardown, context);
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
	}
}
