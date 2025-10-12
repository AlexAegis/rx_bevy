use crate::{ObservableSubscription, SubscriptionLike, WithSubscriptionContext};

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
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync;

	/// ScheduledHandles can be turned into UnscheduledHandles. This type here
	/// allows the [SubscriptionContext][crate::SubscriptionContext] to ensure
	/// only one UnscheduledHandle type is used. That turning a ScheduledHandle
	/// into an UnscheduledHandle will result in the same type as when creating
	/// a new UnscheduledHandle directly from the
	/// UnscheduledSubscriptionAllocator defined on the
	/// [SubscriptionContext][crate::SubscriptionContext].
	type UnscheduledHandle<Subscription>: UnscheduledSubscriptionHandle<Context = Self::Context>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_scheduled_subscription<Subscription>(
		subscription: Subscription,
		context: &mut Self::Context,
	) -> Self::ScheduledHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync;
}

/// # ScheduledSubscriptionHandle
///
/// The main handle for subscriptions that need scheduling. Only one can exist
/// for a single subscription and is the owner of that subscription.
///
/// These types do not implement [Clone], instead, they have their own `clone`
/// method that returns an [UnscheduledSubscriptionHandle], ensuring only one
/// reference exists for the subscription that can be scheduled.
///
/// By calling `downgrade` on such handle, one can acquire a clonable,
/// non-owning "weak" handle that can be used to unsubscribe the subscription.
pub trait ScheduledSubscriptionHandle: ObservableSubscription + Send + Sync {
	type UnscheduledHandle: UnscheduledSubscriptionHandle<Context = Self::Context>;
	type WeakHandle: WeakSubscriptionHandle<Context = Self::Context>;

	fn downgrade(&mut self) -> Self::WeakHandle;

	/// To ensure only one handle is scheduled, this "fake" clone method returns
	/// an [UnscheduledHandle][ScheduledSubscriptionHandle::UnscheduledHandle].
	fn clone(&self) -> Self::UnscheduledHandle;
}

/// # WeakSubscriptionHandle
///
/// These are clonable handles for
/// [ObservableSubscription][crate::ObservableSubscription]s and other
/// handles that own a subscription, allowing them to be unsubscribed from
/// multiple places without preventing them to be dropped, or to
/// allowing other places to erroneously tick it.
///
/// Can be acquired by calling [`downgrade`][ScheduledSubscriptionHandle::downgrade]
/// on a [ScheduledSubscriptionHandle] or on an [UnscheduledSubscriptionHandle].
///
/// ## Note To Implementors
///
/// While this trait is empty I want you to explicitly declare a type meant to
/// be used for this as it has to align with some expected behavior
///
/// - It must not unsubscribe on drop, as these are not owners of the
///   subscription they point to.
/// - It most not be tickable. Only the main "strong" handle can be tickable,
///   and that one is not allowed to be cloned.
pub trait WeakSubscriptionHandle: SubscriptionLike + Clone + Send + Sync {}

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

/// # UnscheduledSubscriptionHandle
///
/// An owning handle for subscriptions that must not have scheduling. Can be
/// cloned to prevent the underlying subscription from dropping.
///
/// > These are mainly meant for the ConnectableObservable's connection which
/// > should not be ticked, it just represents an active connection.
pub trait UnscheduledSubscriptionHandle: SubscriptionLike + Clone + Send + Sync {
	type WeakHandle: WeakSubscriptionHandle<Context = Self::Context>;

	fn downgrade(&mut self) -> Self::WeakHandle;
}
