use std::sync::{Arc, RwLock};

use crate::{
	Observable, ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf,
	PrimaryCategorySubscriber, Subscriber, SubscriptionLike, TeardownCollection, Tick, Tickable,
	WithPrimaryCategory,
	allocator::ErasedSharedDestination,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
};

impl<S> WithSubscriptionContext for Arc<RwLock<S>>
where
	S: ?Sized + WithSubscriptionContext,
{
	type Context = S::Context;
}

impl<Destination> WithPrimaryCategory for Arc<RwLock<Destination>>
where
	Destination: ?Sized + WithPrimaryCategory,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for Arc<RwLock<Destination>> where
	Destination: ?Sized + ObserverUpgradesToSelf
{
}

impl<Destination> ErasedSharedDestination for Arc<RwLock<Destination>> where
	Destination: 'static + ?Sized + Subscriber + Send + Sync
{
}

impl<Destination> ObserverInput for Arc<RwLock<Destination>>
where
	Destination: ?Sized + ObserverInput,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SharedDestination<Destination> for Arc<RwLock<Destination>>
where
	Destination: 'static + ?Sized + Subscriber + Send + Sync,
{
	fn clone_with_context(
		&self,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		self.clone()
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Ok(destination) = self.read() {
			accessor(&*destination, context)
		}
	}

	fn access_with_context_mut<F>(
		&mut self,
		mut accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Ok(mut destination) = self.write() {
			accessor(&mut *destination, context)
		}
	}
}

impl<Destination> Observer for Arc<RwLock<Destination>>
where
	Destination: ?Sized + Observer + SubscriptionLike,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.next(next, context),
			Err(poison_error) => poison_error.into_inner().unsubscribe(context),
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.error(error, context),
			Err(poison_error) => poison_error.into_inner().unsubscribe(context),
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.complete(context),
			Err(poison_error) => poison_error.into_inner().unsubscribe(context),
		}
	}
}

impl<Destination> Tickable for Arc<RwLock<Destination>>
where
	Destination: ?Sized + Tickable + SubscriptionLike,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match self.write() {
			Ok(mut lock) => lock.tick(tick, context),
			Err(poison_error) => poison_error.into_inner().unsubscribe(context),
		}
	}
}

impl<Destination> SubscriptionLike for Arc<RwLock<Destination>>
where
	Destination: ?Sized + SubscriptionLike,
{
	// Ignore the poison for is_closed checks, so the other signals can still
	// operate and unsubscribe when it's poisoned.
	fn is_closed(&self) -> bool {
		self.read()
			.unwrap_or_else(|err| err.into_inner())
			.is_closed()
	}

	// Ignore the poison on unsubscribe. It's only relevant if you still
	// want to do something with it using the other signals. They will print
	// errors on poison and unsubscribe instead. (And that would cause a double
	// print)
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if self.is_closed() {
			return;
		}

		self.write()
			.unwrap_or_else(|err| err.into_inner())
			.unsubscribe(context)
	}
}

impl<Destination> TeardownCollection for Arc<RwLock<Destination>>
where
	Destination: ?Sized + TeardownCollection + SubscriptionLike,
{
	fn add_teardown(
		&mut self,
		teardown: crate::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match self.write() {
			Ok(mut lock) => {
				lock.add_teardown(teardown, context);
			}
			Err(poison_error) => {
				teardown.execute(context);
				poison_error.into_inner().unsubscribe(context);
			}
		}
	}
}

impl<O> ObservableOutput for Arc<RwLock<O>>
where
	O: ObservableOutput,
{
	type Out = O::Out;
	type OutError = O::OutError;
}

impl<O> Observable for Arc<RwLock<O>>
where
	O: Observable,
{
	type Subscription<Destination>
		= O::Subscription<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ crate::UpgradeableObserver<
				In = Self::Out,
				InError = Self::OutError,
				Context = Self::Context,
			>
			+ Send
			+ Sync,
	{
		let mut destination = destination.upgrade();

		match self.write() {
			Ok(mut lock) => lock.subscribe(destination, context),
			Err(poison_error) => {
				destination.unsubscribe(context);
				panic!("Poisoned lock encountered, unable to subscribe! {poison_error:?}")
			}
		}
	}
}
