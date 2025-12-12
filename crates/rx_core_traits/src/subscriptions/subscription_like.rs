/// A [SubscriptionLike] is something that can be "unsubscribed" from, which
/// will close it, rendering it no longer operational. If it also owns
/// resources, it will also release those resources, usually by executing
/// [Teardown][crate::Teardown]s
///
/// [Drop] is mandatory to manually implement for [TeardownCollection], but not
/// for subscribers that do not actually own a teardown, only forward the
/// added teardowns downstream. This is why this bound is only enforced on the
/// [Observable::Subscription][crate::Observable::Subscription].
///
/// It has to be implemented to guarantee that resources are
/// properly released on drop. Note that some subscriptions do not need to
/// do anything on drop. If that's the case, the `drop` fn should only contain
/// a comment on why it doesn't need to do anything.
pub trait SubscriptionLike {
	/// Returns if the subscription is closed or not. A subscription can be
	/// closed by calling unsubscribe on it. Some special subscriptions made
	/// by observables that only ever emit values during subscribe, will produce
	/// subscriptions that are created closed.
	///
	/// Once closed, a subscription stays closed.
	fn is_closed(&self) -> bool;

	/// Releases all resources associated with this subscription, and marks it
	/// as closed.
	///
	/// Once closed, a subscription stays closed.
	fn unsubscribe(&mut self);
}
