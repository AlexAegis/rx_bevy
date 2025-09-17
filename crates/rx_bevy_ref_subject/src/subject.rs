use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionLike, Tick,
};
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

use crate::Multicast;

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	pub multicast: Arc<RwLock<Multicast<In, InError, Context>>>,
}

impl<In, InError, Context> Clone for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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
	Context: DropContext,
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
	Context: DropContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> SignalContext for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observable for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let multicast = self.multicast.write().expect("asd");
		multicast.subscribe(destination)
	}
}

impl<In, InError, Context> ObserverInput for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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
	Context: DropContext,
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
}

impl<In, InError, Context> Drop for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	// Must not unsubscribe on drop, it's the shared destination that should do that
	fn drop(&mut self) {
		self.unsubscribe(&mut Context::get_context_for_drop());
	}
}
