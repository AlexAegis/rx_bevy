use crate::WithSubscriptionContext;

/// For things that could be derived from its associated context.
///
/// Things that impl [Default] automatically implement [FromContext] and
/// use [Default::default()] to return a value.
pub trait FromContext: WithSubscriptionContext {
	fn from_context(context: &mut Self::Context) -> Self;
}

impl<T> FromContext for T
where
	T: Default + WithSubscriptionContext,
{
	fn from_context(_context: &mut Self::Context) -> Self {
		T::default()
	}
}
