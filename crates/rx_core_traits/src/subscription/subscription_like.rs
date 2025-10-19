use crate::{
	Teardown,
	SubscriptionContext, WithSubscriptionContext,
};

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational.
///
/// [Drop] is mandatory to manually implement for subscriptions, but not for
/// subscribers. This is why this bound is only enforced on the
/// [Observable::Subscription][crate::Observable::Subscription].
///
/// It has to be implemented to guarantee that resources are
/// properly released on drop. Note that some subscriptionlikes do not need to
/// do anything on drop. If that's the case, the `drop` fn should only contain
/// a comment on why it doesn't need to do anything.
pub trait SubscriptionLike: WithSubscriptionContext {
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
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>);

	/// Add additional teardowns to execute on unsubscribe. If the subscription
	/// is already closed, the added teardown is immediately executed!
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add<T>(
		&mut self,
		subscription: T,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		T: Into<Teardown<Self::Context>>,
	{
		let teardown: Teardown<Self::Context> = subscription.into();
		self.add_teardown(teardown, context);
	}

	fn add_fn<F>(
		&mut self,
		f: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: 'static
			+ FnOnce(&mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
			+ Send
			+ Sync,
		Self: Sized,
	{
		let teardown = Teardown::<Self::Context>::new(f);
		self.add(teardown, context);
	}
}

impl<S> SubscriptionCollection for S where S: SubscriptionLike {}
