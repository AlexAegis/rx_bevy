use std::sync::{Arc, RwLock};

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operator, Subscriber, SubscriptionLike,
};
use slab::Slab;

use crate::{MulticastInnerSubscriber, MulticastOuterSubscriber};

pub struct MulticastDestination<In, InError> {
	pub(crate) slab: Slab<Box<dyn Subscriber<In = In, InError = InError>>>,
	pub(crate) closed: bool,
}

impl<In, InError> MulticastDestination<In, InError> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub fn drain(&mut self) -> Vec<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.closed = true;
		self.slab.drain().collect::<Vec<_>>()
	}

	pub fn take(&mut self, key: usize) -> Option<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.slab.try_remove(key)
	}
}

impl<In, InError> Default for MulticastDestination<In, InError> {
	fn default() -> Self {
		Self {
			slab: Slab::with_capacity(1),
			closed: false,
		}
	}
}

pub struct MulticastOperator<In, InError> {
	pub multicast_destination: Arc<RwLock<MulticastDestination<In, InError>>>,
}

impl<In, InError> MulticastOperator<In, InError> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub fn drain(&mut self) -> Vec<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.multicast_destination
			.write()
			.map(|mut multicast_destination| multicast_destination.drain())
			.expect("No poison")
	}

	pub fn take(&mut self, key: usize) -> Option<Box<dyn Subscriber<In = In, InError = InError>>> {
		let mut destination = self.multicast_destination.write().expect("no poison");
		destination.take(key)
	}
}

impl<In, InError> Clone for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn clone(&self) -> Self {
		Self {
			multicast_destination: self.multicast_destination.clone(),
		}
	}
}

impl<In, InError> Default for MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn default() -> Self {
		Self {
			multicast_destination: Arc::new(RwLock::new(MulticastDestination::default())),
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
		let mut multicast_destination = self.multicast_destination.write().expect("Poisoned!");

		let key = {
			let entry = multicast_destination.slab.vacant_entry();
			let key = entry.key();
			let inner_subscriber = MulticastInnerSubscriber {
				destination,
				outer: MulticastOuterSubscriber {
					key,
					subscriber_ref: self.multicast_destination.clone(),
				},
			};
			entry.insert(Box::new(inner_subscriber));
			key
		};

		MulticastOuterSubscriber {
			key,
			subscriber_ref: self.multicast_destination.clone(),
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
		let is_closed = self.is_closed();
		println!("MulticastOperator next 1");

		if !is_closed {
			println!("MulticastOperator next 2");

			if let Ok(mut slab) = self.multicast_destination.write() {
				for (_, destination) in slab.slab.iter_mut() {
					println!("MulticastOperator next 3");

					destination.next(next.clone());
				}
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		let is_closed = self.is_closed();

		if !is_closed {
			if let Ok(mut slab) = self.multicast_destination.write() {
				slab.closed = true;
				for (_, destination) in slab.slab.iter_mut() {
					destination.error(error.clone());
				}
			}
		}
	}

	fn complete(&mut self) {
		let is_closed = self.is_closed();

		println!("MulticastOperator complete 1");

		if !is_closed {
			let mut destinations = self.drain();
			println!("MulticastOperator complete 2");

			for destination in destinations.iter_mut() {
				destination.complete();
			}
		}
	}
}

impl<In, InError> SubscriptionLike for MulticastOperator<In, InError> {
	fn is_closed(&self) -> bool {
		if let Ok(slab) = self.multicast_destination.read() {
			slab.closed
		} else {
			true
		}
	}

	fn unsubscribe(&mut self) {
		println!("MulticastOperator unsubscribe 1");
		let destinations = self.drain();

		println!("    MulticastOperator unsubscribe 2");

		for mut destination in destinations {
			destination.unsubscribe();
		}
	}
}

impl<In, InError> Drop for MulticastOperator<In, InError> {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
