use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SharedDestination,
	Subscriber, SubscriptionLike, Teardown, TeardownCollection, UpgradeableObserver,
	WithPrimaryCategory,
};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
pub struct SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + UpgradeableObserver + Send + Sync,
{
	shared_destination: Arc<Mutex<Destination>>,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + UpgradeableObserver + Send + Sync,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			shared_destination: Arc::new(Mutex::new(destination)),
			_phantom_data: PhantomData,
		}
	}

	pub fn access_with_context<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
		self.shared_destination.access(accessor);
	}

	pub fn access_with_context_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination),
	{
		self.shared_destination.access_mut(accessor);
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
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

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
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
	Destination: 'static + Subscriber + Send + Sync,
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
	Destination: 'static + Subscriber + Send + Sync,
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
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
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
