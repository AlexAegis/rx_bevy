use std::sync::{Arc, RwLock};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Subscriber, Subscription,
	SubscriptionLike, UpgradeableObserver, subscribers::ObserverSubscriber,
};

use crate::{MulticastDestination, MulticastInnerSubscriber, MulticastOuterSubscriber};

/// A Subject is a shared multicast observer, can be used for broadcasting
/// a clone of it still has the same set of subscribers, and is needed if you
/// want to make multiple pipes out of the same subject
pub struct Subject<In, InError = ()>
where
	In: 'static,
	InError: 'static,
{
	pub multicast: Arc<RwLock<MulticastDestination<In, InError>>>,
}

impl<In, InError> Subject<In, InError> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub(crate) fn drain(&mut self) -> Vec<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.multicast
			.write()
			.map(|mut multicast_destination| multicast_destination.drain())
			.expect("No poison")
	}

	pub(crate) fn take(
		&mut self,
		key: usize,
	) -> Option<Box<dyn Subscriber<In = In, InError = InError>>> {
		let mut destination = self.multicast.write().expect("no poison");
		destination.take(key)
	}
}

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<T, Error> Default for Subject<T, Error> {
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(MulticastDestination::default())),
		}
	}
}

impl<T, Error> ObservableOutput for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error> Observable for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	/// TODO: IntoSubscriber!! instead of observer, and plain observers should be into ObserverSubscriber, normal subscribers couldbe then unchanged!!!
	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		d: Destination,
	) -> Subscription {
		let mut multicast_destination = self.multicast.write().expect("Poisoned!");

		let destination = d.upgrade();
		let key = {
			let entry = multicast_destination.slab.vacant_entry();
			let key = entry.key();
			let inner_subscriber = MulticastInnerSubscriber {
				destination,
				outer: MulticastOuterSubscriber {
					key,
					subscriber_ref: self.multicast.clone(),
				},
			};
			entry.insert(Box::new(inner_subscriber));
			key
		};

		let outer = MulticastOuterSubscriber::<Destination::Subscriber> {
			key,
			subscriber_ref: self.multicast.clone(),
		};

		Subscription::new(outer)
	}
}

impl<T, Error> ObserverInput for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	fn next(&mut self, next: Self::In) {
		println!("MulticastOperator next 1");

		if !self.is_closed() {
			println!("MulticastOperator next 2");

			if let Ok(mut multicast) = self.multicast.write() {
				for (_, destination) in multicast.slab.iter_mut() {
					println!("MulticastOperator next 3");

					destination.next(next.clone());
				}
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			if let Ok(mut multicast) = self.multicast.write() {
				multicast.closed = true;
				for (_, destination) in multicast.slab.iter_mut() {
					destination.error(error.clone());
				}
			}
		}
	}

	fn complete(&mut self) {
		println!("MulticastOperator complete 1");

		if !self.is_closed() {
			let mut destinations = self.drain();
			println!("MulticastOperator complete 2");

			for destination in destinations.iter_mut() {
				destination.complete();
			}
		}
	}
}

impl<T, Error> SubscriptionLike for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	fn is_closed(&self) -> bool {
		if let Ok(multicast) = self.multicast.read() {
			multicast.closed
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

impl<T, Error> Drop for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
