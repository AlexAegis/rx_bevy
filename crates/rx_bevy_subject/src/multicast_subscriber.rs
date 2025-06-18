use std::{
	cell::RefCell,
	rc::Rc,
	sync::{Arc, RwLock},
};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operator, Subscriber, SubscriptionLike,
};
use slab::Slab;

use crate::{MulticastInnerSubscriber, MulticastOuterSubscriber};

// TODO: Arc RwLock?
// TODO: Maybe this is actually an Operator??
pub struct MulticastOperator<In, InError> {
	pub destination: Arc<RwLock<Slab<Box<dyn Subscriber<In = In, InError = InError>>>>>,
	// TODO: Maybe this should be a pointer too or be next to the slab
	pub closed: bool,
}

impl<In, InError> Clone for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
			closed: self.closed,
		}
	}
}

impl<In, InError> MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	pub fn new() -> Self {
		Self {
			destination: Arc::new(RwLock::new(Slab::with_capacity(1))),
			closed: false,
		}
	}
}

impl<In, InError> Operator for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>> =
		MulticastOuterSubscriber<Destination>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		let mut slab = self.destination.write().expect("Poisoned!");

		let entry = slab.vacant_entry();
		let key = entry.key();
		let val = MulticastInnerSubscriber {
			destination,
			key,
			subscriber_ref: self.destination.clone(),
		};
		entry.insert(Box::new(val));

		MulticastOuterSubscriber {
			key,
			subscriber_ref: self.destination.clone(),
		}
	}
}

impl<In, InError> ObserverInput for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> Observer for MulticastOperator<In, InError>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	fn next(&mut self, next: In) {
		if !self.closed {
			if let Ok(mut slab) = self.destination.write() {
				for (_, destination) in slab.iter_mut() {
					destination.next(next.clone());
				}
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.closed {
			self.closed = true;
			if let Ok(mut slab) = self.destination.write() {
				for (_, destination) in slab.iter_mut() {
					destination.error(error.clone());
				}
			}
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.closed = true;
			if let Ok(mut slab) = self.destination.write() {
				for (_, destination) in slab.iter_mut() {
					destination.complete();
				}
			}
		}
	}
}

impl<In, InError> SubscriptionLike for MulticastOperator<In, InError> {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
		if let Ok(mut slab) = self.destination.write() {
			for mut destination in slab.drain() {
				destination.unsubscribe();
			}
		}
	}
}

impl<In, InError> Drop for MulticastOperator<In, InError> {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
