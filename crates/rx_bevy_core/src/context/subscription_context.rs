use crate::{DestinationSharer, ErasedDestinationSharer, SignalBound, Subscriber};

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
	type Sharer<Destination>: DestinationSharer<In = Destination::In, InError = Destination::InError, Context = Self>
	where
		Destination: 'static + Subscriber<Context = Self> + Send + Sync;

	/// Defines how a new subscription should be created for erased subscribers
	/// that can create additional subscriptions as they operate.
	type ErasedSharer<In, InError>: ErasedDestinationSharer<In = In, InError = InError, Context = Self>
	where
		In: SignalBound,
		InError: SignalBound;

	fn create_context_to_unsubscribe_on_drop() -> Self;
}

mod private {
	pub trait Seal {}
}

pub trait SubscriptionContextDropSafety: private::Seal + 'static {
	/// Boolean to indicate if this context is safe to create during a drop
	const DROP_SAFE: bool;
}

/// Marker struct for Contexts that **CANNOT** be acquired out of thin air, for
/// the purposes of unsubscribing a [SubscriptionLike].
///
/// These contexts **MUST** panic when `get_context_for_drop` is called.
/// For some contexts this is natural, as it is un-implementable. For example
/// if it contains references.
/// But even if it could theoretically be constructed, but it would not actually
/// facilitate the cleanup of resources - because those resources can only
/// be referenced through the context - it should still panic, as it would
/// cause a memory leak.
///
/// An example of a [DropUnsafeSubscriptionContext] is any Context used to interface
/// with an ECS system where the only way of freeing up (and acquiring)
/// resources is through the context, which holds a reference of that short
/// lived object that lets you interact with the ECS.
pub struct DropUnsafeSubscriptionContext;

impl private::Seal for DropUnsafeSubscriptionContext {}

impl SubscriptionContextDropSafety for DropUnsafeSubscriptionContext {
	const DROP_SAFE: bool = false;
}

/// Marker struct for Contexts that **CAN** be acquired out of thin air, for
/// the purposes of unsubscribing a [SubscriptionLike].
///
/// These contexts **MUST NOT** panic when `get_context_for_drop` is called.
/// By using this marker on your context definition, you promise that calling
/// `get_context_for_drop` not only doesn't panic, but it produces a context
/// that is meaningful for cleaning up resources used by the subscriptions.
///
/// If your context allows you to acquire resources that can only be freed
/// through the context, then it is **NOT** "DropSafe" as that would lead to
/// a memory leak. If that's the case, you should use [DropUnsafeSubscriptionContext]
/// and panic when `get_context_for_drop` is called!
///
/// A trivial example of a [DropSafeSubscriptionContext] is the unit context `()`,
/// because it's empty! It doesn't let you acquire resources outside of the
/// subscription, so you don't need to release anything either!
#[derive(Debug)]
pub struct DropSafeSubscriptionContext;

impl private::Seal for DropSafeSubscriptionContext {}

impl SubscriptionContextDropSafety for DropSafeSubscriptionContext {
	const DROP_SAFE: bool = true;
}
