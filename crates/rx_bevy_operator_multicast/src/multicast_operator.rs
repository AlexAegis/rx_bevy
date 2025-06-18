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
				key,
				multicast_source: self.multicast_destination.clone(),
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
		if !self.is_closed() {
			if let Ok(mut slab) = self.multicast_destination.write() {
				for (_, destination) in slab.slab.iter_mut() {
					destination.next(next.clone());
				}
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			if let Ok(mut slab) = self.multicast_destination.write() {
				slab.closed = true;
				for (_, destination) in slab.slab.iter_mut() {
					destination.error(error.clone());
				}
			}
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			if let Ok(mut slab) = self.multicast_destination.write() {
				slab.closed = true;

				for (_, destination) in slab.slab.iter_mut() {
					destination.complete();
				}
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
		if let Ok(mut slab) = self.multicast_destination.write() {
			slab.closed = true;
			for mut destination in slab.slab.drain() {
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
