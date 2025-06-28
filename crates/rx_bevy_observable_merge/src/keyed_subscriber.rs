use std::sync::{Arc, RwLock};

use rx_bevy_observable::{
	Observable, Observer, ObserverInput, Operation, Subscriber, Subscription, SubscriptionLike,
};
use slab::Slab;

pub struct SubscriptionStore {
	slab: Arc<RwLock<Slab<Subscription>>>,
}

impl Default for SubscriptionStore {
	fn default() -> Self {
		Self {
			slab: Arc::new(RwLock::new(Slab::with_capacity(2))),
		}
	}
}

impl KeyedSubscriptionStore for SubscriptionStore {
	type KeyedSubscriber<Destination: Subscriber> = ManyToOneKeyedSubscriber<Destination>;

	fn is_key_closed(&self, key: usize) -> bool {
		self.slab
			.read()
			.unwrap()
			.get(key)
			.map(|subscription| subscription.is_closed())
			.unwrap_or(true)
	}

	fn unsubscribe_key(&mut self, key: usize) {
		if let Some(mut subscription) = self.slab.write().unwrap().try_remove(key) {
			subscription.unsubscribe();
		}
	}

	fn subscribe_with_store<O: Observable, D: Subscriber<In = O::Out, InError = O::OutError>>(
		&mut self,
		observable: &mut O,
		destination: D,
	) -> (Subscription, usize) {
		let slab_ref = self.slab.clone();
		let mut slab_lock = self.slab.write().unwrap();
		let entry = slab_lock.vacant_entry();
		let key = entry.key();

		let subscriber = Self::KeyedSubscriber::create(destination, key, slab_ref);

		// write lock is still needed at this point, while subscribing, but
		// a read lock is also attempted if the observable completes immediately.
		// Therefore, KeyedSubscribers should always assume that if a lock
		// can't be immediately acquired, for the slab_ref, that the slab is
		// not empty, and not closed.
		let subscription = observable.subscribe(subscriber);

		entry.insert(subscription.clone());

		(subscription, key)
	}
}

pub trait KeyedSubscriber<Destination>: Subscriber
where
	Destination: Subscriber,
{
	fn create(
		destination: Destination,
		key: usize,
		slab_ref: Arc<RwLock<Slab<Subscription>>>,
	) -> Self;
}

impl<Destination> KeyedSubscriber<Destination> for ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn create(
		destination: Destination,
		key: usize,
		slab_ref: Arc<RwLock<Slab<Subscription>>>,
	) -> Self {
		Self {
			destination,
			key,
			slab_ref,
		}
	}
}
pub trait KeyedSubscriptionStore {
	type KeyedSubscriber<Destination: Subscriber>: Subscriber<In = Destination::In, InError = Destination::InError>;

	fn is_key_closed(&self, key: usize) -> bool;
	fn unsubscribe_key(&mut self, key: usize);

	fn subscribe_with_store<
		O: Observable,
		Destination: Subscriber<In = O::Out, InError = O::OutError>,
	>(
		&mut self,
		observable: &mut O,
		destination: Destination,
	) -> (Subscription, usize);
}

pub struct ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub destination: Destination,
	pub key: usize,
	pub slab_ref: Arc<RwLock<Slab<Subscription>>>,
}

impl<Destination> Observer for ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		println!("COMP");
		// TODO: Add another shared thing to track completeness, and check for closedness here, not emptyness.
		// If something is getting written into the shared slab, it's definitely not empty.
		if self
			.slab_ref
			.try_read()
			.map(|slab| slab.is_empty())
			.unwrap_or(false)
		{
			println!("DEST COMP");
			self.destination.complete();
		};
	}
}

impl<Destination> SubscriptionLike for ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.slab_ref.write().unwrap().try_remove(self.key);

		if self
			.slab_ref
			.try_read()
			.map(|slab| slab.is_empty())
			.unwrap_or(false)
		{
			self.destination.unsubscribe();
		}
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<Destination> ObserverInput for ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Operation for ManyToOneKeyedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;
}
