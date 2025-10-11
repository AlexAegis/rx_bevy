use crate::{SubscriptionHandle, TickableSubscription};

pub trait IntoSubscriptionHandle: 'static + TickableSubscription + Sized + Send + Sync {
	fn to_handle(self) -> SubscriptionHandle<Self>;
}

impl<S> IntoSubscriptionHandle for S
where
	S: 'static + TickableSubscription + Sized + Send + Sync,
{
	fn to_handle(self) -> SubscriptionHandle<Self> {
		SubscriptionHandle::new(self)
	}
}
