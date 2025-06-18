use std::{
	cell::RefCell,
	rc::{Rc, Weak},
	sync::{Arc, RwLock},
};

use rx_bevy_observable::{
	Observer, ObserverInput, Operation, Subscriber, SubscriptionLike, Teardown,
};
use slab::Slab;

use crate::MulticastOperator;

pub struct MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	pub(crate) key: usize,
	pub(crate) subscriber_ref: Arc<
		RwLock<
			Slab<
				Box<dyn Subscriber<In = Destination::In, InError = Destination::InError> + 'static>,
			>,
		>,
	>,
}

impl<Destination> ObserverInput for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Operation for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	type Destination = Destination;
}

impl<Destination> Observer for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn next(&mut self, next: Self::In) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.get_mut(self.key) {
				sub.next(next);
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.get_mut(self.key) {
				sub.error(error);
			}
		}
	}

	fn complete(&mut self) {
		if let Ok(mut subscriber) = self.subscriber_ref.write() {
			if let Some(sub) = subscriber.get_mut(self.key) {
				sub.complete();
			}
		}
	}
}

impl<Destination> SubscriptionLike for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn unsubscribe(&mut self) {
		if let Ok(mut subject) = self.subscriber_ref.write() {
			if let Some(destination) = subject.get_mut(self.key) {
				destination.unsubscribe();
				subject.remove(self.key);
			}
		}
	}

	fn is_closed(&self) -> bool {
		if let Ok(subject) = self.subscriber_ref.read() {
			subject
				.get(self.key)
				.map(|destination| destination.is_closed())
				.unwrap_or(!subject.contains(self.key))
		} else {
			true
		}
	}
}

impl<Destination> Drop for MulticastOuterSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}

pub struct MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	pub(crate) destination: Destination,
	pub(crate) key: usize,
	pub(crate) subscriber_ref: Arc<
		RwLock<
			Slab<
				Box<dyn Subscriber<In = Destination::In, InError = Destination::InError> + 'static>,
			>,
		>,
	>,
}

impl<Destination> Operation for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	type Destination = Destination;
}

impl<Destination> SubscriptionLike for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn unsubscribe(&mut self) {
		if let Ok(mut subject) = self.subscriber_ref.write() {
			if let Some(destination) = subject.get_mut(self.key) {
				destination.unsubscribe();
				subject.remove(self.key);
			}
		}
	}

	fn is_closed(&self) -> bool {
		if let Ok(subject) = self.subscriber_ref.read() {
			subject
				.get(self.key)
				.map(|destination| destination.is_closed())
				.unwrap_or(!subject.contains(self.key))
		} else {
			true
		}
	}
}

impl<Destination> ObserverInput for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<Destination> Drop for MulticastInnerSubscriber<Destination>
where
	Destination: 'static + Observer,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
