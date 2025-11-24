use crate::context::{
	SubscriptionContextDropSafety,
	allocator::{
		DestinationAllocator, ErasedDestinationAllocator, ScheduledSubscriptionAllocator,
		UnscheduledSubscriptionAllocator,
	},
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
/// TODO: Rename to SubscriptionAllocationContextProvider?
pub trait SubscriptionContext: 'static {
	/// The actual context reference that will be passed into subscriptions
	/// and subscribers. The lifetime parameters allow for decoupling the
	/// lifetime of the context definition and the lifetime of the context
	/// value passed into subscribers, subscriptions and teardown callbacks.
	///
	/// Historical footage of me figuring out this pattern to enable a trait to be used as a stored function parameter, avoiding lifetime problems https://tenor.com/view/i-have-an-idea-croods-the-croods-grug-gif-16310881167093655388
	///
	/// The only remaining vestige of this crate being bevy related is that
	/// there are two lifetime parameters here instead of just one. If there
	/// would only be one lifetime parameter here, bevy's `'world` and `'state`
	/// lifetimes would merge into one and result downstream in a
	/// `<'world: 'state, 'world: 'state>` lifetime requirement, which is
	/// incompatible with systems.
	type Item<'w, 's>: SubscriptionContextAccess<SubscriptionContextProvider = Self>;

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
	type DestinationAllocator: DestinationAllocator<Context = Self>;

	/// Defines how a new subscription should be created for erased subscribers
	/// that can create additional subscriptions as they operate.
	type ErasedDestinationAllocator: ErasedDestinationAllocator<Context = Self>;

	/// Defines how an [ObservableSubscription][crate::ObservableSubscription]
	/// is turned into a [SubscriptionHandle][crate::SubscriptionHandle] which
	/// can create additional [WeakSubscriptionHandle][crate::WeakSubscriptionHandle]s
	/// that can unsubscribe the handle, but can't tick it.
	type ScheduledSubscriptionAllocator: ScheduledSubscriptionAllocator<Context = Self>;

	type UnscheduledSubscriptionAllocator: UnscheduledSubscriptionAllocator<Context = Self>;

	fn create_context_to_unsubscribe_on_drop<'w, 's>() -> Self::Item<'w, 's>;
}

/// Used as a back reference to the provider
pub trait SubscriptionContextAccess {
	type SubscriptionContextProvider: SubscriptionContext;
}
