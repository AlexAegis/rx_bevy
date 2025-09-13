/// ## Why is there only a single associated context type?
///
/// Both Subscriptions and Observers in the same subscription use the same kind
/// of contexts, as signals have to be able to trigger an unsubscription. Most
/// commonly: completion and error signals should trigger an unsubscribe call.
/// And next signals sometimes trigger completion signals, so all contexts
/// must be the same.
#[doc(alias = "ChannelContext")]
pub trait SignalContext {
	type Context;
}
