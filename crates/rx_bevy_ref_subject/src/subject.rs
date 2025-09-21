use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropContext, DropSafeSignalContext, Observable, ObservableOutput, Observer, ObserverInput,
	SignalContext, Subscriber, SubscriptionLike, Tick,
};

use crate::{Multicast, MulticastSubscription};

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	pub multicast: Arc<RwLock<Multicast<In, InError, Context>>>,
}

impl<In, InError, Context> Clone for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<In, InError, Context> Default for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(Multicast::default())),
		}
	}
}

impl<In, InError, Context> ObservableOutput for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> SignalContext for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<In, InError, Context> Observable for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	{
		let mut multicast = self.multicast.write().expect("asd");
		multicast.subscribe(destination, context)
	}
}

impl<In, InError, Context> ObserverInput for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.complete(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.tick(tick, context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		if let Ok(multicast) = self.multicast.read() {
			multicast.is_closed()
		} else {
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.unsubscribe(context);
		}
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::Context::get_context_for_drop()
	}
}

impl<In, InError, Context> Drop for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}
