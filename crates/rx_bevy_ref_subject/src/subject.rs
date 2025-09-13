use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, TeardownFn, Tick,
};
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

use crate::MulticastDestination;

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<'c, In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	pub multicast: Arc<RwLock<MulticastDestination<'c, In, InError, Context>>>,
}

impl<'c, In, InError, Context> Subject<'c, In, InError, Context>
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

impl<'c, In, InError, Context> Clone for Subject<'c, In, InError, Context>
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

impl<'c, In, InError, Context> Default for Subject<'c, In, InError, Context>
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

impl<'c, In, InError, Context> ObservableOutput for Subject<'c, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Out = In;
	type OutError = InError;
}

impl<'c, In, InError, Context> Observable<'c> for Subject<'c, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<'c, Context>;

	fn subscribe<Destination>(
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

		// TODO: Big problem, due to 'c on multicast and observable (this) bound together
		let multicast_ref = self.multicast.clone();
		DropSubscription::new_from::<TeardownFn<<Self::Subscription as SignalContext>::Context>>(
			move |_: &mut <Self::Subscription as SignalContext>::Context| {
				let subscriber = {
					let mut write_multicast = multicast_ref.write().expect("blocked 1");
					write_multicast.take(key)
				};

				if let Some(mut subscriber) = subscriber {
					subscriber.unsubscribe(&mut Context::get_context_for_drop());
				}
			},
			context,
		)
	}
}

impl<'c, In, InError, Context> ObserverInput for Subject<'c, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<'c, In, InError, Context> SignalContext for Subject<'c, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<'c, In, InError, Context> Observer for Subject<'c, In, InError, Context>
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

impl<'c, In, InError, Context> SubscriptionLike for Subject<'c, In, InError, Context>
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

impl<'c, In, InError, Context> SubscriptionCollection<'c> for Subject<'c, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn add<S: 'c + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		if let Ok(mut multicast) = self.multicast.write() {
			multicast.add(subscription, context);
		}
	}
}

impl<'c, In, InError, Context> Drop for Subject<'c, In, InError, Context>
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
