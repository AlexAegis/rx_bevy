use crate::{
	DestinationSharer, ErasedDestinationSharer, ObservableSubscription,
	ScheduledSubscriptionAllocator, SignalBound, Subscriber, SubscriptionContextDropSafety,
	SubscriptionLike, UnscheduledSubscriptionAllocator,
};

/// ## Why is there only a single associated context type?
///
/// Both Subscriptions and Observers in the same subscription use the same kind
/// of contexts, as signals have to be able to trigger an unsubscription. Most
/// commonly: completion and error signals should trigger an unsubscribe call.
/// And next signals sometimes trigger completion signals, so all contexts
/// must be the same.
pub trait WithSubscriptionContext {
	type Context: SubscriptionContext;
}

/// ## [SubscriptionContext][crate::SubscriptionContext]
///
/// The context defines how new subscriptions can be acquired in an observable.
///
/// TODO: Rename to SubscriptionAllocationContext
pub trait SubscriptionContext {
	/// Indicates if the context can be safely (or not) acquired during a drop
	/// to perform a last minute unsubscription in case the subscription is not
	/// already closed.
	///
	/// Certain subscribers or subscriptions may demand a context that is
	/// safe to drop subscriptions with, without requiring the user to manually
	/// unsubscribe everything that happens to go out of scope. While also
	/// providing a mechanic to environments where unsubscription at drop is
	/// impossible, but going out of scope isn't a concern because it provides
	/// hooks for when that would happen, like in an ECS.
	type DropSafety: SubscriptionContextDropSafety;

	/// Defines how a new subscription should be created for subscribers that
	/// can create additional subscriptions as they operate.
	///
	/// TODO: Maybe call these Allocators? SubscriberAllocator/SubscriptionAllocator
	type Sharer<Destination>: DestinationSharer<In = Destination::In, InError = Destination::InError, Context = Self>
	where
		Destination: 'static + Subscriber<Context = Self> + Send + Sync;

	/// Defines how a new subscription should be created for erased subscribers
	/// that can create additional subscriptions as they operate.
	type ErasedSharer<In, InError>: ErasedDestinationSharer<In = In, InError = InError, Context = Self>
	where
		In: SignalBound,
		InError: SignalBound;

	/// Defines how an [ObservableSubscription][crate::ObservableSubscription]
	/// is turned into a [SubscriptionHandle][crate::SubscriptionHandle] which
	/// can create additional [WeakSubscriptionHandle][crate::WeakSubscriptionHandle]s
	/// that can unsubscribe the handle, but can't tick it.
	type ScheduledSubscriptionAllocator<Subscription>: ScheduledSubscriptionAllocator<
		Context = Self,
		UnscheduledHandle<Subscription> = <Self::UnscheduledSubscriptionAllocator<Subscription> as UnscheduledSubscriptionAllocator>::UnscheduledHandle<Subscription>
	>
	where
		Subscription: 'static + ObservableSubscription<Context = Self> + Send + Sync;

	type UnscheduledSubscriptionAllocator<Subscription>: UnscheduledSubscriptionAllocator<
		Context = Self,
	>
	where
		Subscription: 'static + SubscriptionLike<Context = Self> + Send + Sync;

	fn create_context_to_unsubscribe_on_drop() -> Self;
}
