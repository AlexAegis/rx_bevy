use core::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SharedDestination,
	Subscriber, SubscriptionLike, Teardown, TeardownCollection, UpgradeableObserver,
	WithPrimaryCategory,
};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
#[derive(Debug)]
pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	shared_destination: Arc<Mutex<Destination>>,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			shared_destination: Arc::new(Mutex::new(destination)),
			_phantom_data: PhantomData,
		}
	}

	/// Locks the shared destination.
	/// In case it encounters a poison error, the destination is immediately
	/// unsubscribed!
	pub fn lock(&self) -> MutexGuard<'_, Destination> {
		self.shared_destination
			.lock()
			.unwrap_or_else(|poison_error| {
				let mut destination = poison_error.into_inner();
				destination.unsubscribe();
				destination
			})
	}

	pub fn downgrade(&self) -> Weak<Mutex<Destination>> {
		Arc::downgrade(&self.shared_destination)
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithPrimaryCategory for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for SharedSubscriber<Destination> where
	Destination: Subscriber + Send + Sync
{
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			shared_destination: self.shared_destination.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.shared_destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.shared_destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.shared_destination.complete();
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.shared_destination.unsubscribe();
	}
}

impl<Destination> TeardownCollection for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
	}
}

impl<Destination> SharedDestination<Destination> for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		self.shared_destination.access(accessor);
	}

	#[inline]
	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		self.shared_destination.access_mut(accessor);
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		// Should not unsubscribe on drop as it's shared!
	}
}
