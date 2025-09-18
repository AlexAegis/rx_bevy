use crate::Subscriber;

/// A SharedSubscriber is a subscriber that guarantees that if you clone it,
/// the signals sent to the clone will reach the same recipient as the original
/// subscriber did.
pub trait ShareableSubscriber<Destination>:
	Subscriber<In = Destination::In, InError = Destination::InError, Context = Destination::Context>
where
	Destination: 'static + Subscriber,
{
	type Shared: Subscriber<
			In = Destination::In,
			InError = Destination::InError,
			Context = Destination::Context,
		> + Clone;

	fn share(destination: Destination) -> Self::Shared;
}
