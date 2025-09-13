use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	InnerSubscription, ObserverInput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike,
};
use slab::Slab;

use crate::MulticastSubscriber;

pub struct MulticastDestination<'c, In, InError, Context> {
	pub(crate) slab: Slab<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>>,
	pub(crate) closed: bool,
	pub(crate) teardown: InnerSubscription<'c, Context>,
}

impl<'c, In, InError, Context> ObserverInput for MulticastDestination<'c, In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<'c, In, InError, Context> MulticastDestination<'c, In, InError, Context> {
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

impl<'c, In, InError, Context> Default for MulticastDestination<'c, In, InError, Context> {
	fn default() -> Self {
		Self {
			slab: Slab::with_capacity(1),
			closed: false,
			teardown: InnerSubscription::default(),
		}
	}
}

impl<'c, In, InError, Context> SignalContext for MulticastDestination<'c, In, InError, Context> {
	type Context = Context;
}

impl<'c, In, InError, Context> SubscriptionLike for MulticastDestination<'c, In, InError, Context> {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.teardown.unsubscribe(context);
	}
}

impl<'c, In, InError, Context> SubscriptionCollection<'c>
	for MulticastDestination<'c, In, InError, Context>
{
	fn add<S: 'c + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.teardown.add(subscription, context);
	}
}
