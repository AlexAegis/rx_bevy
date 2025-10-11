use crate::{ObservableSubscription, SubscriptionHandle};

pub trait IntoSubscriptionHandle: 'static + ObservableSubscription + Sized + Send + Sync {
	fn into_handle(self) -> SubscriptionHandle<Self>;
}

impl<S> IntoSubscriptionHandle for S
where
	S: 'static + ObservableSubscription + Sized + Send + Sync,
{
	fn into_handle(self) -> SubscriptionHandle<Self> {
		SubscriptionHandle::new(self)
	}
}
