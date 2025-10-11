use crate::{ObserverInput, SignalContext, Subscriber, WithContext};

/// An [ErasedDestinationSharer] that can create an [ErasedSharedDestination]
/// out of a destination.
///
/// Mainly used by subjects.
pub trait ErasedDestinationSharer: ObserverInput + WithContext {
	type Shared: ErasedSharedDestination<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination, context: &mut Self::Context) -> Self::Shared
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync;
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

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context);

	fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context);
}

pub trait ErasedDestinationSharedTypes: 'static + Subscriber {
	type Sharer: ErasedDestinationSharer<In = Self::In, InError = Self::InError, Context = Self::Context>;
	type Shared: ?Sized
		+ ErasedSharedDestination<In = Self::In, InError = Self::InError, Context = Self::Context>;
	type Access: ?Sized
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

impl<Destination> ErasedDestinationSharedTypes for Destination
where
	Destination: Subscriber + 'static,
{
	type Sharer =
		<Self::Context as SignalContext>::ErasedSharer<Destination::In, Destination::InError>;
	type Shared = <Self::Sharer as ErasedDestinationSharer>::Shared;
	type Access = <Self::Shared as ErasedSharedDestination>::Access;
}
