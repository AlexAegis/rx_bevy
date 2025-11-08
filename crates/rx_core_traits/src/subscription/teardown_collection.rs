use crate::{SubscriptionContext, Teardown, WithSubscriptionContext};

/// A [TeardownCollection] is something that owns resources that can be
/// released through the [SubscriptionLike] traits `unsubscribe` method.
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
pub trait TeardownCollection: WithSubscriptionContext {
	/// Add additional teardowns to execute on unsubscribe. If the subscription
	/// is already closed, the added teardown is immediately executed!
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

pub trait TeardownCollectionExtension: TeardownCollection {
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

impl<S> TeardownCollectionExtension for S where S: TeardownCollection {}
