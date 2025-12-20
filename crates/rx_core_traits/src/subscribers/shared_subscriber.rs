use std::sync::{Arc, Mutex, MutexGuard, Weak};

use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{SharedDestination, Subscriber, UpgradeableObserver};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
#[derive(Debug, RxSubscriber)]
#[_rx_core_traits_crate(crate)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	#[destination]
	shared_destination: Arc<Mutex<Destination>>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber + UpgradeableObserver + Send + Sync,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			shared_destination: Arc::new(Mutex::new(destination)),
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

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			shared_destination: self.shared_destination.clone(),
		}
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
