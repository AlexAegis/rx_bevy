use crate::{
	SignalBound, Subscriber,
	SubscriptionContext, WithSubscriptionContext,
};

/// An [ErasedSubscriberAllocator] that can create an [ErasedSharedDestination]
/// out of a destination.
///
/// Mainly used by subjects.
pub trait ErasedDestinationAllocator: WithSubscriptionContext {
	type Shared<In, InError>: ErasedSharedDestination<In = In, InError = InError, Context = Self::Context>
	where
		In: SignalBound,
		InError: SignalBound;

	fn share<Destination>(
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;
}

/// An [ErasedSharedDestination] is a subscriber that can be cloned, where each
/// clone points to the exact same destination subscriber.
///
/// Since they always define a layer on the destination they share, an
/// [`access`][SharedDestination::access] method is provided to inspect the
/// destination it wraps. In the case of an `Arc<RwLock<Destination>>` calling
/// the `access_mut` method will attempt to write lock the destination for the
/// duration of the call.
pub trait ErasedSharedDestination: Subscriber + Clone + Send + Sync {}

pub trait ErasedSharedDestinationTypes: 'static + Subscriber {
	type Sharer: ErasedDestinationAllocator<Context = Self::Context>;
	type Shared: ?Sized
		+ ErasedSharedDestination<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

impl<Destination> ErasedSharedDestinationTypes for Destination
where
	Destination: Subscriber + 'static,
{
	type Sharer = <Self::Context as SubscriptionContext>::ErasedDestinationAllocator;
	type Shared =
		<Self::Sharer as ErasedDestinationAllocator>::Shared<Destination::In, Destination::InError>;
}
