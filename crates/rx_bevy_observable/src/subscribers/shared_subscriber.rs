use std::sync::{Arc, RwLock};

use crate::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

pub struct SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Arc<RwLock<Destination>>,
}

impl<Destination> From<Destination> for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(destination)),
		}
	}

	pub fn new_from_shared(destination: impl Into<Arc<RwLock<Destination>>>) -> Self {
		Self {
			destination: destination.into(),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Destination),
	{
		reader(&self.destination.read().expect("poisoned"))
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut Destination),
	{
		reader(&mut self.destination.write().expect("poisoned"))
	}
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.complete();
		}
	}
}

impl<Destination> SubscriptionLike for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		let lock = self.destination.read().expect("lock is poisoned!");
		lock.is_closed()
	}

	fn unsubscribe(&mut self) {
		let mut lock = self.destination.write().expect("lock is poisoned!");
		lock.unsubscribe();
	}
}

impl<Destination> Drop for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	/// Should not unsubscribe on drop as it's shared
	fn drop(&mut self) {}
}

impl<Destination> Operation for SharedSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;
}
