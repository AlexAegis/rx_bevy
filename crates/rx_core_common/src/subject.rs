use crate::{
	Observable, ObserverInput, PrimaryCategorySubject, RxObserver, SubscriptionLike,
	UpgradeableObserver,
};

/// A Subject is something that is an Observable and Observer (Subscriber) at
/// the same time. Signals pushed into it will be received by the subscriptions
/// made from it, broadcasting them.
///
/// Subjects do not actually implement [SubscriptionLike][crate::SubscriptionLike]
/// despite them having an `unsubscribe` method and being closable.
/// The reason is that subjects do not and can't own resources, that's the job
/// of the subscriptions you make with them. This allows subjects to be safely
/// droppable without having to call `unsubscribe` on them even in a
/// [DropUnsafeSubscriptionContext][crate::DropUnsafeSubscriptionContext].
/// The `unsubscribe` method on subjects is only there for users to mass
/// unsubscribe every subscription made from the subjects.
pub trait SubjectLike:
	Observable<PrimaryCategory = PrimaryCategorySubject>
	+ RxObserver
	+ SubscriptionLike
	+ UpgradeableObserver
{
}

impl<T> SubjectLike for T
where
	T: Observable<PrimaryCategory = PrimaryCategorySubject>
		+ RxObserver
		+ SubscriptionLike
		+ UpgradeableObserver,
	<T as ObserverInput>::In: Clone,
	<T as ObserverInput>::InError: Clone,
{
}
