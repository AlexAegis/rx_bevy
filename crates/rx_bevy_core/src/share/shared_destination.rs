use crate::{ObserverInput, SubscriptionContext, Subscriber, WithSubscriptionContext};

/// A [DestinationSharer] that can create a [SharedDestination] out of a
/// destination subscriber.
pub trait DestinationSharer: ObserverInput + WithSubscriptionContext {
	type Shared<Destination>: SharedDestination<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync;

	fn share<Destination>(
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Shared<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Send
			+ Sync;
}

/// A [SharedDestination] is a subscriber that can be cloned, where each clone
/// points to the exact same destination subscriber.
///
/// Different [SharedDestination]s behave differently, some are just simply
/// smart pointers with locks, some are reference counted on a subscriber level
/// and unsubscribe when the last clone unsubscribes even before all clones are
/// dropped, like with a regular [Rc].
///
/// Since they always define a layer on the destination they share, an
/// [`access`][SharedDestination::access] method is provided to inspect the
/// destination it wraps. In the case of an `ErasedArcSubscriber` calling
/// the `access_mut` method will attempt to write lock the destination for the
/// duration of the call.
pub trait SharedDestination<Destination>:
	Subscriber<In = Destination::In, InError = Destination::InError, Context = Destination::Context>
	+ Clone
	+ Send
	+ Sync
where
	Destination: ?Sized + 'static + Subscriber,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination);

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination);

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Destination, &mut Self::Context);

	fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Destination, &mut Self::Context);
}

pub trait DestinationSharedTypes: 'static + Subscriber {
	type Sharer: DestinationSharer<In = Self::In, InError = Self::InError, Context = Self::Context>;
	type Shared: ?Sized + SharedDestination<Self>;
}

impl<Destination> DestinationSharedTypes for Destination
where
	Destination: Subscriber + 'static,
{
	type Sharer = <Self::Context as SubscriptionContext>::Sharer<Self>;
	type Shared = <Self::Sharer as DestinationSharer>::Shared<Self>;
}
