use crate::{Teardown, WithContext};

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
pub trait SubscriptionLike: WithContext {
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
	fn unsubscribe(&mut self, context: &mut Self::Context);

	/// Add additional teardowns to execute on unsubscribe. If the subscription
	/// is already closed, the added teardown is immediately executed!
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context);

	/// In case the subscription wasn't closed when it got dropped, it will
	/// try to unsubscribe, as it must be guaranteed that a subscription
	/// releases all its resources, otherwise a memory leak would occur.
	/// For this unsubscribe to happen, a context must be provided.
	///
	/// If this is implemented for a Subscriber, it should just call the
	/// destinations `get_context_to_unsubscribe_on_drop` and leave the rest to
	/// it. If it's implemented for a Subscription, this function provides a
	/// chance to create a context from the Subscription itself, if that can't
	/// be done, you should use the [SignalContext][crate::SignalContext]s
	/// [`create_context_to_unsubscribe_on_drop`][crate::SignalContext::create_context_to_unsubscribe_on_drop]
	/// function. This function, depending on the context used can panic!
	///
	/// Some trivial contexts, like the unit `()` context when a context isn't
	/// needed, are always safe and will never panic when you drop a
	/// subscription. But in an ECS context when the resources associated with
	/// the subscription are stored in the ECS, a context is needed to release
	/// those resources, and if that reference can't be accessed globally, a
	/// panic must happen.
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context;
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add<T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		T: Into<Teardown<Self::Context>>,
	{
		let teardown: Teardown<Self::Context> = subscription.into();
		self.add_teardown(teardown, context);
	}

	fn add_fn<F>(&mut self, f: F, context: &mut Self::Context)
	where
		F: 'static + FnOnce(&mut Self::Context) + Send + Sync,
		Self: Sized,
	{
		let teardown = Teardown::<Self::Context>::new(f);
		self.add(teardown, context);
	}
}

impl<S> SubscriptionCollection for S where S: SubscriptionLike {}
