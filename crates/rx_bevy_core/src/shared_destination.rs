use crate::{ObserverInput, SignalContext, Subscriber};

pub trait DestinationSharer: ObserverInput + SignalContext {
	type Shared<Destination>: SharedDestination<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(destination: Destination) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

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
