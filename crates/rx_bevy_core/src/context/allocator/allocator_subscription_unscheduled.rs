use crate::{
	SubscriptionLike,
	context::{WithSubscriptionContext, allocator::handle::UnscheduledSubscriptionHandle},
};

/// # ScheduledSubscriptionAllocator
///
/// A type that can create a [ScheduledSubscriptionHandle] from an
/// [ObservableSubscription][crate::ObservableSubscription], taking ownership
/// of the subscription.
pub trait UnscheduledSubscriptionAllocator: WithSubscriptionContext {
	type UnscheduledHandle<Subscription>: UnscheduledSubscriptionHandle<Context = Self::Context>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<Subscription>(
		subscription: Subscription,
		context: &mut Self::Context,
	) -> Self::UnscheduledHandle<Subscription>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;
}
