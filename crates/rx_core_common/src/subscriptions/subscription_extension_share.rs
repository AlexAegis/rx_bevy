use crate::{SharedSubscription, SubscriptionLike};

pub trait SubscriptionLikeExtensionIntoShared
where
	Self: 'static + SubscriptionLike + Sized + Send + Sync,
{
	/// Wrap this subscription into a [SharedSubscription], erasing it and
	/// allowing you to freely clone it, to unsubscribe it from multiple places.
	fn into_shared(self) -> SharedSubscription {
		SharedSubscription::new(self)
	}
}

impl<Subscription> SubscriptionLikeExtensionIntoShared for Subscription where
	Subscription: 'static + SubscriptionLike + Sized + Send + Sync
{
}
