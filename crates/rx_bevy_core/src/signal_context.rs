/// ## Why is there only a single associated context type?
///
/// Both Subscriptions and Observers in the same subscription use the same kind
/// of contexts, as signals have to be able to trigger an unsubscription. Most
/// commonly: completion and error signals should trigger an unsubscribe call.
/// And next signals sometimes trigger completion signals, so all contexts
/// must be the same.
/// TODO: Maybe a better name would be Environment, or ExecutionEnvironment
#[doc(alias = "ChannelContext")]
pub trait SignalContext {
	type Context: DropContext;
}

/// In addition to [ContextFromSubscription], this trait denotes contexts for
/// for dropped [Subscription]s. For example when the context is just `()`.
///
/// If a type can't implement this it should Panic
/// TODO: Give it a more generic name, this is required for all contexts, Use SignalContext here, and rename the other one
pub trait DropContext {
	/// Indicates if the context can be safely (or not) acquired during a drop
	/// to perform a last minute unsubscription in case the subscription is not
	/// already closed.
	/// Certain subscribers or subscriptions may demand a context that is
	/// safe to drop subscriptions with without requiring the user to manually
	/// unsubscribe everything that happens to go out of scope. While providing
	/// a mechanic to environments where unsubscription at drop is impossible,
	/// but going out of scope isn't a concern because it provides hooks for
	/// when that would happen, like in an ECS.
	type DropSafety: SignalContextDropSafety;

	fn get_context_for_drop() -> Self;
}

mod private {
	pub trait Seal {}
}

pub trait SignalContextDropSafety: private::Seal + 'static {
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
/// An example of a [DropUnsafeSignalContext] is any Context used to interface
/// with an ECS system where the only way of freeing up (and acquiring)
/// resources is through the context, which holds a reference of that short
/// lived object that lets you interact with the ECS.
pub struct DropUnsafeSignalContext;

impl private::Seal for DropUnsafeSignalContext {}

impl SignalContextDropSafety for DropUnsafeSignalContext {
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
/// a memory leak. If that's the case, you should use [DropUnsafeSignalContext]
/// and panic when `get_context_for_drop` is called!
///
/// A trivial example of a [DropSafeSignalContext] is the unit context `()`,
/// because it's empty! It doesn't let you acquire resources outside of the
/// subscription, so you don't need to release anything either!
pub struct DropSafeSignalContext;

impl private::Seal for DropSafeSignalContext {}

impl SignalContextDropSafety for DropSafeSignalContext {
	const DROP_SAFE: bool = true;
}
