use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropContext, DropSubscription, Observable, ObservableOutput, Observer, ObserverInput,
	SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

use crate::MulticastDestination;

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = (), Context = ()>
where
	In: 'static,
	InError: 'static,
{
	pub multicast: Arc<RwLock<MulticastDestination<In, InError, Context>>>,
}

impl<In, InError, Context> Subject<In, InError, Context> {
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

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<T, Error> Default for Subject<T, Error> {
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(MulticastDestination::default())),
		}
	}
}

impl<T, Error> ObservableOutput for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error, Context> Observable for Subject<T, Error, Context>
where
	T: 'static,
	Error: 'static,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<'c, Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination: Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	{
		let subscriber = destination;

		let mut multicast_destination = self.multicast.write().expect("Poisoned!");

		let key = multicast_destination
			.multicast_subscribe::<Destination>(subscriber, self.multicast.clone());

		let multicast_ref = self.multicast.clone();
		DropSubscription::new(Teardown::new(Box::new(move |_| {
			let subscriber = {
				let mut write_multicast = multicast_ref.write().expect("blocked 1");
				write_multicast.take(key)
			};

			if let Some(mut subscriber) = subscriber {
				subscriber.unsubscribe();
			}
		})))
	}
}

impl<T, Error, Context> ObserverInput for Subject<T, Error, Context>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	type In = T;
	type InError = Error;
}

impl<T, Error, Context> SignalContext for Subject<T, Error, Context>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	type Context = Context;
}

impl<T, Error, Context> Observer for Subject<T, Error, Context>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			for (_, destination) in multicast.slab.iter_mut() {
				destination.next(next.clone());
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.closed = true;
			for (_, destination) in multicast.slab.iter_mut() {
				destination.error(error.clone());
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut destinations = self.close_and_drain();
			for destination in destinations.iter_mut() {
				destination.complete();
			}
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			for (_, destination) in multicast.slab.iter_mut() {
				destination.tick(tick.clone());
			}
		}
	}
}

impl<T, Error, Context> SubscriptionLike for Subject<T, Error, Context>
where
	T: 'static,
	Error: 'static,
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

impl<T, Error, Context> SubscriptionCollection for Subject<T, Error, Context>
where
	T: 'static,
	Error: 'static,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		if let Ok(mut multicast) = self.multicast.write() {
			multicast.add(subscription, context);
		}
	}
}

impl<T, Error> Drop for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	// Must not unsubscribe on drop, it's the shared destination that should do that
	fn drop(&mut self) {}
}
