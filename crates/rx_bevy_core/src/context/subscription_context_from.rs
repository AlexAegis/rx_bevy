use crate::context::{SubscriptionContext, WithSubscriptionContext};

/// For things that could be derived from its associated context.
///
/// Things that impl [Default] automatically implement [FromContext] and
/// use [Default::default()] to return a value.
pub trait FromSubscriptionContext: WithSubscriptionContext {
	fn from_context(context: &mut <Self::Context as SubscriptionContext>::Item<'_>) -> Self;
}

impl<T> FromSubscriptionContext for T
where
	T: Default + WithSubscriptionContext,
{
	fn from_context(_context: &mut <Self::Context as SubscriptionContext>::Item<'_>) -> Self {
		T::default()
	}
}
