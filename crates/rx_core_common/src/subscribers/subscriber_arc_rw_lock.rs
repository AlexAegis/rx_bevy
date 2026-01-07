use std::sync::{Arc, RwLock};

use crate::{
	Observable, ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf,
	PrimaryCategorySubscriber, SharedDestination, Subscriber, SubscriptionLike, Teardown,
	TeardownCollection, WithPrimaryCategory,
};

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
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		if let Ok(destination) = self.read() {
			accessor(&destination)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Ok(mut destination) = self.write() {
			accessor(&mut destination)
		}
	}
}

impl<Destination> Observer for Arc<RwLock<Destination>>
where
	Destination: ?Sized + Observer + SubscriptionLike,
{
	fn next(&mut self, next: Self::In) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.next(next),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn error(&mut self, error: Self::InError) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.error(error),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn complete(&mut self) {
		if self.is_closed() {
			return;
		}

		match self.write() {
			Ok(mut lock) => lock.complete(),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
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

	// Ignore poison on unsubscribe; it only matters if other signals still need
	// it. They already log poison errors and unsubscribe instead, which would
	// otherwise double print.
	fn unsubscribe(&mut self) {
		if self.is_closed() {
			return;
		}

		self.write()
			.unwrap_or_else(|err| err.into_inner())
			.unsubscribe()
	}
}

impl<Destination> TeardownCollection for Arc<RwLock<Destination>>
where
	Destination: ?Sized + TeardownCollection + SubscriptionLike,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		match self.write() {
			Ok(mut lock) => {
				lock.add_teardown(teardown);
			}
			Err(poison_error) => {
				teardown.execute();
				poison_error.into_inner().unsubscribe();
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
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ crate::UpgradeableObserver<In = Self::Out, InError = Self::OutError>
			+ Send
			+ Sync,
	{
		let destination = destination.upgrade();

		match self.write() {
			Ok(mut lock) => lock.subscribe(destination),
			Err(poison_error) => {
				let mut subscription = poison_error.into_inner().subscribe(destination);
				subscription.unsubscribe();
				subscription
			}
		}
	}
}
