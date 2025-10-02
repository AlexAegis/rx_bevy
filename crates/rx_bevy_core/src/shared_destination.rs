use crate::{ObserverInput, SignalContext, Subscriber};

pub trait SharedDestination:
	Subscriber<
		In = <Self::Access as ObserverInput>::In,
		InError = <Self::Access as ObserverInput>::InError,
		Context = <Self::Access as SignalContext>::Context,
	> + Clone
{
	type Access: ?Sized + Subscriber;

	fn share<D>(destination: D) -> Self
	where
		Self::Access: Sized,
		D: 'static
			+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
			+ Into<Self::Access>;

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
	Sharer: SharedDestination,
{
	|_: Sharer| ()
}
