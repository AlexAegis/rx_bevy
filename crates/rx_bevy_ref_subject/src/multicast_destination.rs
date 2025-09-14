use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	InnerSubscription, ObserverInput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown,
};
use slab::Slab;

use crate::MulticastSubscriber;

pub struct MulticastDestination<In, InError, Context> {
	pub(crate) slab: Slab<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>>,
	pub(crate) closed: bool,
	pub(crate) teardown: InnerSubscription<Context>,
}

impl<In, InError, Context> ObserverInput for MulticastDestination<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> MulticastDestination<In, InError, Context> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub fn drain(
		&mut self,
	) -> Vec<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>> {
		self.closed = true;
		self.slab.drain().collect::<Vec<_>>()
	}

	pub fn take(
		&mut self,
		key: usize,
	) -> Option<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>> {
		self.slab.try_remove(key)
	}

	pub fn multicast_subscribe<
		Destination: 'static + Subscriber<In = In, InError = InError, Context = Context>,
	>(
		&mut self,
		subscriber: Destination,
		subscriber_ref: Arc<RwLock<MulticastDestination<In, InError, Context>>>,
	) -> usize {
		let entry = self.slab.vacant_entry();
		let key = entry.key();
		let subscriber = MulticastSubscriber::<Destination> {
			key,
			destination: subscriber,
			subscriber_ref,
		};
		entry.insert(Box::new(subscriber));
		key
	}
}

impl<In, InError, Context> Default for MulticastDestination<In, InError, Context> {
	fn default() -> Self {
		Self {
			slab: Slab::with_capacity(1),
			closed: false,
			teardown: InnerSubscription::default(),
		}
	}
}

impl<In, InError, Context> SignalContext for MulticastDestination<In, InError, Context> {
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for MulticastDestination<In, InError, Context> {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.teardown.unsubscribe(context);
	}
}

impl<In, InError, Context> SubscriptionCollection for MulticastDestination<In, InError, Context> {
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.teardown.add(subscription, context);
	}
}
