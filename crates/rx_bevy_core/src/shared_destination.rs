use crate::{ObserverInput, Subscriber, WithContext};

/// A [DestinationSharer] that can create a [SharedDestination] out of a
/// destination subscriber.
pub trait DestinationSharer: ObserverInput + WithContext {
	type Shared<Destination>: SharedDestination<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

/// A [SharedDestination] is a subscriber that can be cloned, but each clone
/// will point to the exact same destination subscriber.
///
/// Different [SharedDestination]s behave differently, some are just simply
/// smart pointers with locks, some are reference counted on a subscriber level
/// and unsubscribe when the last clone unsubscribes even before all clones are
/// dropped, like with a regular [Rc].
///
/// Since the always define a layer on the destination they share, an
/// [`access`][SharedDestination::access] method is provided to inspect the
/// destination it wraps. In the case of an `Arc<RwLock<Destination>>` calling
/// the `access_mut` method will attempt to write lock the destination for the
/// duration of the call.
pub trait SharedDestination<Destination>:
	Subscriber<In = Destination::In, InError = Destination::InError, Context = Destination::Context>
	+ Clone
where
	Destination: ?Sized + 'static + Subscriber,
{
	type Access: ?Sized
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn access<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context);

	fn access_mut<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context);
}

/// Convenience function to define a sharer from a function argument position, it's a noop and will never get called.
pub fn use_sharer<Sharer>() -> impl Fn(Sharer)
where
	Sharer: DestinationSharer,
{
	|_: Sharer| ()
}
