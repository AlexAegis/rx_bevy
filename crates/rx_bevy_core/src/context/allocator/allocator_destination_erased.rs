use crate::{
	SignalBound, Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext},
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
pub trait ErasedSharedDestination: Subscriber + Clone + Send + Sync {
	type Access: ?Sized
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Self::Access);

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Self::Access);

	fn access_with_context<F>(&mut self, accessor: F, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
	where
		F: Fn(&Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>);

	fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
	where
		F: FnMut(&mut Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>);
}

pub trait ErasedSharedDestinationTypes: 'static + Subscriber {
	type Sharer: ErasedDestinationAllocator<Context = Self::Context>;
	type Shared: ?Sized
		+ ErasedSharedDestination<In = Self::In, InError = Self::InError, Context = Self::Context>;
	type Access: ?Sized
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

impl<Destination> ErasedSharedDestinationTypes for Destination
where
	Destination: Subscriber + 'static,
{
	type Sharer = <Self::Context as SubscriptionContext>::ErasedDestinationAllocator;
	type Shared =
		<Self::Sharer as ErasedDestinationAllocator>::Shared<Destination::In, Destination::InError>;
	type Access = <Self::Shared as ErasedSharedDestination>::Access;
}
