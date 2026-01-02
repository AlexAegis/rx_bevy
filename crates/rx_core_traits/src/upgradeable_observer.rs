use crate::{ObserverInput, Subscriber};

/// When a subscription is established, the destination must only receive the
/// `next`, `error` and `complete` signals, but not `unsubscribe`. And since
/// subscribers and subjects are observers too, they could be passed as a
/// destination, but doing it naively would always unsubscribe them too when
/// the observable unsubscribes.
///
/// To control this behavior, observers can define in what form they should be
/// used when used as a destination.
///
/// As a rule of thumb this is how different kind of things should behave:
/// - Observer: Must only receive `next`, `error` and `complete` signals.
///   This is only true observers that are **only** observers. That do not
///   implement SubscriptionLike to become Subscribers.
/// - Subjects: Same as Observer. Even though they implement SubscriptionLike,
///   that is meant for external control for the user, and should not be called
///   by upstream.
/// - Subscriber: Should forward everything. Subscribers are only used for Pipes
///   and Operators, and they must be fully connected.
///
/// To prevent erroneously missing calling `upgrade` in an Observable's
/// subscribe function, [UpgradeableObserver] does NOT have [Observer] as its
/// supertrait!
pub trait UpgradeableObserver: ObserverInput + Send + Sync {
	type Upgraded: Subscriber<In = Self::In, InError = Self::InError>;

	fn upgrade(self) -> Self::Upgraded;
}

impl<T> UpgradeableObserver for T
where
	T: Subscriber + ObserverUpgradesToSelf,
{
	type Upgraded = T;

	fn upgrade(self) -> Self::Upgraded {
		self
	}
}

/// To mark types that upgrade to themselves when used as a destination in an
/// Observables subscribe method. Usually subscribers fall into this category
/// as they are already subscribers, so there's no need for an upgrade. But
/// there are a few regular observers too that chose to implement
/// SubscriptionLike to implement a unique behavior. These observers take it
/// upon themselves to manage their own closing logic, and maintain a teardown
/// for added teardowns.
/// For example, the PrintObserver does this to be able to print when it got
/// unsubscribed, and the MockObserver to be able to track all notifications
/// for assertions.
pub trait ObserverUpgradesToSelf {}
