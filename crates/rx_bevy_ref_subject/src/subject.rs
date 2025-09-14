use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

use crate::MulticastDestination;

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	pub multicast: Arc<RwLock<MulticastDestination<In, InError, Context>>>,
}

impl<In, InError, Context> Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	/// Closes the multicast and drains its subscribers to be unsubscribed.
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub(crate) fn close_and_drain(
		&mut self,
	) -> Vec<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>> {
		let mut multicast = self.multicast.write().expect("poison");
		multicast.closed = true;
		multicast.drain()
	}
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
			multicast: Arc::new(RwLock::new(MulticastDestination::default())),
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
		context: &mut Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let subscriber = destination;

		let mut multicast_destination = self.multicast.write().expect("Poisoned!");

		let key = multicast_destination
			.multicast_subscribe::<Destination>(subscriber, self.multicast.clone());

		let multicast_ref = self.multicast.clone();
		let mut s = DropSubscription::default();
		s.add_fn(
			move |c| {
				let subscriber = {
					let mut write_multicast = multicast_ref.write().expect("blocked 1");
					write_multicast.take(key)
				};
				if let Some(mut subscriber) = subscriber {
					subscriber.unsubscribe(c);
				}
			},
			context,
		);
		s
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
			for (_, destination) in multicast.slab.iter_mut() {
				destination.next(next.clone(), context);
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.closed = true;
			for (_, destination) in multicast.slab.iter_mut() {
				destination.error(error.clone(), context);
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut destinations = self.close_and_drain();
			for destination in destinations.iter_mut() {
				destination.complete(context);
			}
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			for (_, destination) in multicast.slab.iter_mut() {
				destination.tick(tick.clone(), context);
			}
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
			multicast.closed
		} else {
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		for mut destination in self.close_and_drain() {
			destination.unsubscribe(context);
		}
	}
}

impl<In, InError, Context> SubscriptionCollection for Subject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		if let Ok(mut multicast) = self.multicast.write() {
			multicast.add(subscription, context);
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
