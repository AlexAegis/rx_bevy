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

/// 🦭
mod private {
	pub trait Seal {}
}
