use std::sync::{Arc, RwLock};

use crate::{Observer, ObserverInput, Operation, Subscription};

/// A simple wrapper for a plain [Observer] to make it "closeable"
#[derive(Debug)]
pub struct ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub destination: Destination,
	pub closed: bool,
}

impl<Destination> ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
		}
	}
}

impl<Destination> Observer for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
		}
	}
}

impl<Destination> ObserverInput for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Subscription for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
	}
}

impl<Destination> Operation for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	type Destination = Destination;
}

impl<Destination> From<Destination> for ObserverSubscriber<Destination>
where
	Destination: Observer,
{
	fn from(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
		}
	}
}

// Maybe this should be a shared subscriber?
pub struct SharedSubscriber<Destination>
where
	Destination: Observer,
{
	destination: Arc<RwLock<ObserverSubscriber<Destination>>>,
}

impl<Destination> SharedSubscriber<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(ObserverSubscriber {
				destination,
				closed: false,
			})),
		}
	}

	pub fn new_from_shared(destination: Arc<RwLock<ObserverSubscriber<Destination>>>) -> Self {
		Self {
			destination: destination.clone(),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&Destination),
	{
		reader(&self.destination.read().unwrap().destination)
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut Destination),
	{
		reader(&mut self.destination.write().unwrap().destination)
	}
}

impl<Destination> Clone for SharedSubscriber<Destination>
where
	Destination: Observer,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedSubscriber<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for SharedSubscriber<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.closed = true;
			lock.destination.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.closed = true;
			lock.destination.complete();
		}
	}
}

impl<Destination> Subscription for SharedSubscriber<Destination>
where
	Destination: Observer,
{
	fn is_closed(&self) -> bool {
		let lock = self.destination.read().expect("lock is poisoned!");
		lock.closed
	}

	fn unsubscribe(&mut self) {
		let mut lock = self.destination.write().expect("lock is poisoned!");
		lock.closed = true;
	}
}
