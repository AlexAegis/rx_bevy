use crate::{Observable, Observer, PrimaryCategorySubject, SubscriptionLike, UpgradeableObserver};

/// A Subject is something that is an Observable and Observer (Subscriber) at
/// the same time. Signals pushed into it will be received by the subscriptions
/// made from it, broadcasting them.
pub trait SubjectLike:
	Observable<PrimaryCategory = PrimaryCategorySubject>
	+ Observer
	+ SubscriptionLike
	+ UpgradeableObserver
{
}

impl<T> SubjectLike for T where
	T: Observable<PrimaryCategory = PrimaryCategorySubject>
		+ Observer
		+ SubscriptionLike
		+ UpgradeableObserver
{
}
