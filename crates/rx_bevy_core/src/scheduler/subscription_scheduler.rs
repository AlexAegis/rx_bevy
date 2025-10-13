use crate::{
	Observable, ObservableSubscription, Subscriber, Tickable,
	context::{
		SubscriptionContext, SubscriptionIntoScheduledHandle,
		allocator::{ScheduledSubscriptionAllocator, handle::ScheduledSubscriptionHandle},
	},
};

/// A [SubscriptionScheduler] holds ScheduledHandles (the only unique reference
/// to a subscription that can be ticked) and ticks them.
pub trait SubscriptionScheduler: Tickable {
	fn schedule<Subscription>(
		&mut self,
		subscription: <<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::ScheduledHandle<Subscription>,
	) where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync;
}

pub trait ScheduleOnSubscribe<Scheduler>: Observable {
	fn subscribe_on<Destination>(
		&mut self,
		destination: Destination,
		scheduler: &mut Scheduler,
		context: &mut Self::Context,
	) -> <<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::UnscheduledHandle<Self::Subscription>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;
}

impl<O, Scheduler> ScheduleOnSubscribe<Scheduler> for O
where
	O: Observable,
	O::Subscription: Send + Sync,
	Scheduler: SubscriptionScheduler<Context = Self::Context>,
{
	fn subscribe_on<Destination>(
		&mut self,
		destination: Destination,
		scheduler: &mut Scheduler,
		context: &mut Self::Context,
	) -> <<Self::Context as SubscriptionContext>::ScheduledSubscriptionAllocator as ScheduledSubscriptionAllocator>::UnscheduledHandle<Self::Subscription>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let subscription = self.subscribe(destination, context);
		let scheduled_handle = subscription.into_scheduled_handle(context);
		let unscheduled_handle = scheduled_handle.clone();
		scheduler.schedule(scheduled_handle);
		unscheduled_handle
	}
}
