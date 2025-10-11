use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalBound, SignalContext, Subscriber,
	SubscriptionHandle, SubscriptionLike, Teardown, Tick, Tickable, WithContext,
};

use crate::{Multicast, MulticastSubscription};

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = (), Context = ()>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	pub multicast: Arc<RwLock<Multicast<In, InError, Context>>>,
}

impl<In, InError, Context> Clone for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
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
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(Multicast::default())),
		}
	}
}

impl<In, InError, Context> ObservableOutput for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> WithContext for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observable for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut multicast = self.multicast.write().expect("asd");
		multicast.subscribe(destination, context)
	}
}

impl<In, InError, Context> ObserverInput for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
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
}

impl<In, InError, Context> Tickable for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if let Ok(mut multicast) = self.multicast.write() {
			multicast.tick(tick, context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
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

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.add_teardown(teardown, context);
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Self::Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<In, InError, Context> Drop for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}
