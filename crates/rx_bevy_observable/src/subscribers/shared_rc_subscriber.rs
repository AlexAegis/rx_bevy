use std::sync::{Arc, RwLock};

use crate::{Observer, ObserverInput, Operation, Subscriber, SubscriptionLike};

/// Does not do reference counting by itself, use [SharedRcSubscriber]
pub struct RcDestination<Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	/// Starts from 1
	ref_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	completion_count: usize,
	/// Starts from 0 if the destination is not already closed, otherwise 1
	unsubscribe_count: usize,

	closed: bool,
}

impl<Destination> RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn new(destination: Destination) -> Self {
		let (new_count, new_closed) = if destination.is_closed() {
			(1, true)
		} else {
			(0, false)
		};

		Self {
			destination,
			ref_count: 1,
			completion_count: new_count,
			unsubscribe_count: new_count,
			closed: new_closed,
		}
	}

	pub fn unsubscribe_if_can(&mut self) {
		if self.unsubscribe_count == self.ref_count && !self.closed {
			self.closed = true;
			self.destination.unsubscribe()
		}
	}

	pub fn complete_if_can(&mut self) {
		if self.completion_count == self.ref_count && !self.closed {
			self.closed = true;
			self.destination.complete()
		}
	}
}

impl<Destination> ObserverInput for RcDestination<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			self.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.completion_count += 1;
			self.complete_if_can();
		}
	}
}

impl<Destination> SubscriptionLike for RcDestination<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.unsubscribe_count += 1;
			self.unsubscribe_if_can();
		}
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<Destination> Drop for RcDestination<Destination>
where
	Destination: Subscriber,
{
	/// When dropped, the reference counts don't matter, the destination has to be unsubscribed.
	/// This should only happen when all counters reach 0.
	fn drop(&mut self) {
		debug_assert_eq!(self.completion_count, 0);
		debug_assert_eq!(self.unsubscribe_count, 0);
		debug_assert_eq!(self.ref_count, 0);

		self.destination.unsubscribe();
	}
}

/// TODO: Maybe instead of a weak boolean, a different type would be more correct, and simpler, for now fuck it
pub struct SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	destination: Arc<RwLock<RcDestination<Destination>>>,
	weak: bool,
	completed: bool,
	unsubscribed: bool,
}

impl<Destination> From<Destination> for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(RcDestination::new(destination))),
			weak: false,
			completed: false,
			unsubscribed: false,
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&RcDestination<Destination>),
	{
		reader(&self.destination.read().expect("poisoned"))
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read_mut<F>(&mut self, mut reader: F)
	where
		F: FnMut(&mut RcDestination<Destination>),
	{
		reader(&mut self.destination.write().expect("poisoned"))
	}

	/// Clone without incrementing the reference counter
	pub fn weak_clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			weak: true,
			completed: self.completed,
			unsubscribed: self.unsubscribed,
		}
	}
}

impl<Destination> Clone for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		if !self.weak {
			let mut destination = self.destination.write().expect("lock is poisoned!");
			destination.ref_count += 1;

			if self.completed {
				destination.completion_count += 1;
			}

			if self.unsubscribed {
				destination.unsubscribe_count += 1;
			}
		};
		Self {
			weak: self.weak,
			completed: self.completed,
			unsubscribed: self.completed,
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for SharedRcSubscriber<Destination>
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
			self.completed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.complete();
		}
	}
}

impl<Destination> SubscriptionLike for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.completed || self.unsubscribed
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.unsubscribed = true;
			let mut lock = self.destination.write().expect("lock is poisoned!");
			lock.unsubscribe();
		}
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		let mut lock = self.destination.write().expect("lock is poisoned!");
		lock.add(subscription);
	}
}

impl<Destination> Drop for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		let mut lock = self.destination.write().expect("lock is poisoned!");

		if !self.weak {
			lock.ref_count -= 1;

			if self.completed {
				lock.completion_count -= 1;
			}

			if self.unsubscribed {
				lock.unsubscribe_count -= 1;
			}
		}

		lock.complete_if_can();
		lock.unsubscribe_if_can();
	}
}

impl<Destination> Operation for SharedRcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Destination = Destination;
}
