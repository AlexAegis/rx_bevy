use crate::Subscriber;

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
pub trait ShareableSubscriber: Subscriber {
	type Shared<Destination>: Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
		+ Clone
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

/// TODO: IDEA A noop just to define S, where a shareable needs to be defined
pub fn use_share<S>()
where
	S: ShareableSubscriber,
{
}
