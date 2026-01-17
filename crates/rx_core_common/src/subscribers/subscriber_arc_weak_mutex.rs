use std::sync::{Mutex, Weak};

use crate::{
	Observable, ObservableOutput, ObserverInput, ObserverUpgradesToSelf, OptionSubscription,
	PrimaryCategorySubscriber, RxObserver, SharedDestination, Subscriber, SubscriptionLike,
	Teardown, TeardownCollection, WithPrimaryCategory,
};

impl<Destination> WithPrimaryCategory for Weak<Mutex<Destination>>
where
	Destination: ?Sized + WithPrimaryCategory,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for Weak<Mutex<Destination>> where
	Destination: ?Sized + ObserverUpgradesToSelf
{
}

impl<Destination> ObserverInput for Weak<Mutex<Destination>>
where
	Destination: ?Sized + ObserverInput,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> SharedDestination<Destination> for Weak<Mutex<Destination>>
where
	Destination: 'static + ?Sized + Subscriber + Send + Sync,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		if let Some(upgraded) = self.upgrade()
			&& let Ok(destination) = upgraded.lock()
		{
			accessor(&destination)
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Some(upgraded) = self.upgrade()
			&& let Ok(mut destination) = upgraded.lock()
		{
			accessor(&mut destination)
		}
	}
}

impl<Destination> RxObserver for Weak<Mutex<Destination>>
where
	Destination: ?Sized + RxObserver + SubscriptionLike,
{
	fn next(&mut self, next: Self::In) {
		let Some(upgraded) = self.upgrade() else {
			return;
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.next(next),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn error(&mut self, error: Self::InError) {
		let Some(upgraded) = self.upgrade() else {
			return;
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.error(error),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn complete(&mut self) {
		let Some(upgraded) = self.upgrade() else {
			return;
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.complete(),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}
}

impl<Destination> SubscriptionLike for Weak<Mutex<Destination>>
where
	Destination: ?Sized + SubscriptionLike,
{
	// Ignore the poison for is_closed checks, so the other signals can still
	// operate and unsubscribe when it's poisoned.
	fn is_closed(&self) -> bool {
		if let Some(upgraded) = self.upgrade() {
			upgraded
				.lock()
				.unwrap_or_else(|err| err.into_inner())
				.is_closed()
		} else {
			true
		}
	}

	// Ignore poison on unsubscribe; it only matters if other signals still need
	// it. They already log poison errors and unsubscribe instead, which would
	// otherwise double print.
	fn unsubscribe(&mut self) {
		let Some(upgraded) = self.upgrade() else {
			return;
		};

		upgraded
			.lock()
			.unwrap_or_else(|err| err.into_inner())
			.unsubscribe()
	}
}

impl<Destination> TeardownCollection for Weak<Mutex<Destination>>
where
	Destination: ?Sized + TeardownCollection + SubscriptionLike,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		let Some(upgraded) = self.upgrade() else {
			teardown.execute();
			return;
		};

		match upgraded.lock() {
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

impl<O> ObservableOutput for Weak<Mutex<O>>
where
	O: ObservableOutput,
{
	type Out = O::Out;
	type OutError = O::OutError;
}

impl<O> Observable for Weak<Mutex<O>>
where
	O: Observable,
{
	type Subscription<Destination>
		= OptionSubscription<O::Subscription<Destination>>
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

		let Some(upgraded) = self.upgrade() else {
			return OptionSubscription::new(None);
		};

		OptionSubscription::new(Some(match upgraded.lock() {
			Ok(mut lock) => lock.subscribe(destination),
			Err(poison_error) => {
				let mut subscription = poison_error.into_inner().subscribe(destination);
				subscription.unsubscribe();
				subscription
			}
		}))
	}
}
