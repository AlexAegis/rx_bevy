use crate::{Subscriber, SubscriptionCollection};

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
pub trait ShareableSubscriber: Subscriber {
	type Shared<Destination>: Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
		+ SubscriptionCollection
		+ Clone
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ SubscriptionCollection;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ SubscriptionCollection;
}

/// Convenience function to define a sharer from a function argument position, it's a noop and will never get called.
pub fn use_share<Sharer>() -> impl Fn(Sharer) -> ()
where
	Sharer: ShareableSubscriber,
{
	|_: Sharer| ()
}
