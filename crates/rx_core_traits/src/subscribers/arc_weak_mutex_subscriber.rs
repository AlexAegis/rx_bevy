use std::sync::{Mutex, Weak};

use disqualified::ShortName;

use crate::{
	Observable, ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf,
	PrimaryCategorySubscriber, SharedDestination, Subscriber, SubscriptionLike, Teardown,
	TeardownCollection, WithPrimaryCategory,
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
		if let Some(upgraded) = self.upgrade() {
			if let Ok(destination) = upgraded.lock() {
				accessor(&destination)
			}
		}
	}

	fn access_mut<F>(&mut self, mut accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		if let Some(upgraded) = self.upgrade() {
			if let Ok(mut destination) = upgraded.lock() {
				accessor(&mut destination)
			}
		}
	}
}

impl<Destination> Observer for Weak<Mutex<Destination>>
where
	Destination: ?Sized + Observer + SubscriptionLike,
{
	fn next(&mut self, next: Self::In) {
		if self.is_closed() {
			return;
		}

		let Some(upgraded) = self.upgrade() else {
			return;
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.next(next),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn error(&mut self, error: Self::InError) {
		if self.is_closed() {
			return;
		}

		let Some(upgraded) = self.upgrade() else {
			return;
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.error(error),
			Err(poison_error) => poison_error.into_inner().unsubscribe(),
		}
	}

	fn complete(&mut self) {
		if self.is_closed() {
			return;
		}

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

	// Ignore the poison on unsubscribe. It's only relevant if you still
	// want to do something with it using the other signals. They will print
	// errors on poison and unsubscribe instead. (And that would cause a double
	// print)
	fn unsubscribe(&mut self) {
		if self.is_closed() {
			return;
		}

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
		let mut destination = destination.upgrade();

		let Some(upgraded) = self.upgrade() else {
			panic!(
				"Tried to subscribe to a weak reference of observabe {}. But it was dropped!",
				ShortName::of::<O>()
			)
		};

		match upgraded.lock() {
			Ok(mut lock) => lock.subscribe(destination),
			Err(poison_error) => {
				destination.unsubscribe();
				panic!("Poisoned lock encountered, unable to subscribe! {poison_error:?}")
			}
		}
	}
}
