use crate::{Observable, Subscriber, SubscriptionCollection, SubscriptionLike};

pub trait SubjectLike<Subscription>: Clone + Observable<Subscription> + Subscriber
where
	Subscription: SubscriptionLike + SubscriptionCollection,
{
}

impl<T, Subscription> SubjectLike<Subscription> for T
where
	T: Clone + Observable<Subscription> + Subscriber,
	Subscription: SubscriptionLike + SubscriptionCollection,
{
}
