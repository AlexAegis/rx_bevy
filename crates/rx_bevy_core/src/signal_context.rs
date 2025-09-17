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
	fn get_context_for_drop() -> Self;
}
