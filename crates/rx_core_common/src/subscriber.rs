use crate::{ObserverUpgradesToSelf, RxObserver, SubscriptionWithTeardown};

/// # [Subscriber]
///
/// A [Subscriber] is an [RxObserver] that is also a
/// [SubscriptionLike][crate::SubscriptionLike], so it can clean itself up upon
/// unsubscribe.
///
/// ## For Implementations
///
/// A struct implementing [Subscriber] should have all their fields as private,
/// as users will never directly interact with a [Subscriber].
///
/// ### Inlining
///
/// A subscribers [RxObserver] functions like `next`, `error` and `complete`
/// that just simply forward the signal to its destination should always
/// be `#[inline]`.
pub trait Subscriber:
	RxObserver + ObserverUpgradesToSelf + SubscriptionWithTeardown + Send + Sync
{
}

impl<T> Subscriber for T where
	T: RxObserver + ObserverUpgradesToSelf + SubscriptionWithTeardown + Send + Sync
{
}
