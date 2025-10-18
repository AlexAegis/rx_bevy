use crate::{
	ObservableSubscription, SubscriptionLike,
	context::{
		SubscriptionContext,
		allocator::{ScheduledSubscriptionAllocator, UnscheduledSubscriptionAllocator},
	},
};

pub trait SubscriptionIntoScheduledHandle: ObservableSubscription + Sized + Send + Sync {
	fn into_scheduled_handle(self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) -> <<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::ScheduledHandle<Self>;
}

impl<S> SubscriptionIntoScheduledHandle for S
where
	S: ObservableSubscription + Sized + Send + Sync,
{
	fn into_scheduled_handle(self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) -> <<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::ScheduledHandle<Self>{
		<<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::allocate_scheduled_subscription(self, context)
	}
}

pub trait SubscriptionIntoUnscheduledHandle: SubscriptionLike + Sized + Send + Sync {
	fn into_unscheduled_handle(self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) -> <<Self::Context as SubscriptionContext>::UnscheduledSubscriptionAllocator as UnscheduledSubscriptionAllocator>::UnscheduledHandle<Self>;
}

impl<S> SubscriptionIntoUnscheduledHandle for S
where
	S: SubscriptionLike + Sized + Send + Sync,
{
	fn into_unscheduled_handle(self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) -> <<Self::Context as SubscriptionContext>::UnscheduledSubscriptionAllocator as UnscheduledSubscriptionAllocator>::UnscheduledHandle<Self>{
		<<Self::Context as SubscriptionContext>::UnscheduledSubscriptionAllocator as UnscheduledSubscriptionAllocator>::allocate_unscheduled_subscription(self, context)
	}
}
