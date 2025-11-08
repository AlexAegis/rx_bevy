use crate::SubscriptionWithTeardown;

use super::WeakSubscriptionHandle;

/// # UnscheduledSubscriptionHandle
///
/// An owning handle for subscriptions that must not have scheduling. Can be
/// cloned to prevent the underlying subscription from dropping.
///
/// > These are mainly meant for the ConnectableObservable's connection which
/// > should not be ticked, it just represents an active connection.
pub trait UnscheduledSubscriptionHandle: SubscriptionWithTeardown + Clone + Send + Sync {
	type WeakHandle: WeakSubscriptionHandle<Context = Self::Context>;

	fn downgrade(&mut self) -> Self::WeakHandle;
}
