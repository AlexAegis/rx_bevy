use crate::{
	SubscriptionScheduled, SubscriptionWithTeardown,
	context::{
		WithSubscriptionContext,
		allocator::handle::{ScheduledSubscriptionHandle, UnscheduledSubscriptionHandle},
	},
};

/// # ScheduledSubscriptionAllocator
///
/// A type that can create a [ScheduledSubscriptionHandle] from an
/// [ObservableSubscription][crate::ObservableSubscription], taking ownership
/// of the subscription.
pub trait ScheduledSubscriptionAllocator: WithSubscriptionContext {
	/// Unique handle that can be scheduled. Can be downgraded into a
	/// [WeakSubscriptionHandle].
	type ScheduledHandle<Subscription>: ScheduledSubscriptionHandle<
			Context = Self::Context,
			UnscheduledHandle = Self::UnscheduledHandle<Subscription>,
		>
	where
		Subscription: 'static + SubscriptionScheduled<Context = Self::Context> + Send + Sync;

	/// ScheduledHandles can be turned into UnscheduledHandles. This type here
	/// allows the [SubscriptionContext][crate::SubscriptionContext] to ensure
	/// only one UnscheduledHandle type is used. That turning a ScheduledHandle
	/// into an UnscheduledHandle will result in the same type as when creating
	/// a new UnscheduledHandle directly from the
	/// UnscheduledSubscriptionAllocator defined on the
	/// [SubscriptionContext][crate::SubscriptionContext].
	type UnscheduledHandle<Subscription>: UnscheduledSubscriptionHandle<Context = Self::Context>
	where
		Subscription: 'static + SubscriptionWithTeardown<Context = Self::Context> + Send + Sync;
}
