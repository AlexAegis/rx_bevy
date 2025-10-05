use crate::{ObserverInput, SignalContext, Subscriber};

pub trait SharedDestination:
	Subscriber<
		In = <Self::Access as ObserverInput>::In,
		InError = <Self::Access as ObserverInput>::InError,
		Context = <Self::Access as SignalContext>::Context,
	> + Clone
{
	type Access: ?Sized + Subscriber;

	type Shared<D>: SharedDestination
	where
		D: 'static
			+ Subscriber<
				In = <Self::Access as ObserverInput>::In,
				InError = <Self::Access as ObserverInput>::InError,
				Context = <Self::Access as SignalContext>::Context,
			>;

	fn share<D>(destination: D) -> Self::Shared<D>
	where
		D: 'static
			+ Subscriber<
				In = <Self::Access as ObserverInput>::In,
				InError = <Self::Access as ObserverInput>::InError,
				Context = <Self::Access as SignalContext>::Context,
			>;

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
