use crate::Teardown;

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
pub trait TeardownCollection {
	/// Add additional teardowns to execute on unsubscribe. If the subscription
	/// is already closed, the added teardown is immediately executed!
	fn add_teardown(&mut self, teardown: Teardown);
}

pub trait TeardownCollectionExtension: TeardownCollection {
	fn add<T>(&mut self, subscription: T)
	where
		T: Into<Teardown>,
	{
		let teardown: Teardown = subscription.into();
		self.add_teardown(teardown);
	}

	fn add_fn<F>(&mut self, f: F)
	where
		F: 'static + FnOnce() + Send + Sync,
		Self: Sized,
	{
		let teardown = Teardown::new(f);
		self.add(teardown);
	}
}

impl<S> TeardownCollectionExtension for S where S: TeardownCollection {}
