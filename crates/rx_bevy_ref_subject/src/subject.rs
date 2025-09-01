use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, Subscriber, Subscription,
	SubscriptionLike, Teardown, UpgradeableObserver,
};

use crate::MulticastDestination;

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
pub struct Subject<In, InError = ()>
where
	In: 'static,
	InError: 'static,
{
	pub multicast: Arc<RwLock<MulticastDestination<In, InError>>>,
}

impl<In, InError> Subject<In, InError> {
	/// Closes the multicast and drains its subscribers to be unsubscribed.
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub(crate) fn close_and_drain(
		&mut self,
	) -> Vec<Box<dyn Subscriber<In = In, InError = InError>>> {
		let mut multicast = self.multicast.write().expect("poison");
		multicast.closed = true;
		multicast.drain()
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
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let subscriber = destination.upgrade();

		let mut multicast_destination = self.multicast.write().expect("Poisoned!");

		let key = multicast_destination
			.multicast_subscribe::<Destination>(subscriber, self.multicast.clone());

		let multicast_ref = self.multicast.clone();
		Subscription::new(Teardown::new(Box::new(move || {
			let subscriber = {
				let mut write_multicast = multicast_ref.write().expect("blocked 1");
				write_multicast.take(key)
			};

			if let Some(mut subscriber) = subscriber {
				subscriber.unsubscribe();
			}
		})))
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
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			for (_, destination) in multicast.slab.iter_mut() {
				destination.next(next.clone());
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			multicast.closed = true;
			for (_, destination) in multicast.slab.iter_mut() {
				destination.error(error.clone());
			}
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			let mut destinations = self.close_and_drain();
			for destination in destinations.iter_mut() {
				destination.complete();
			}
		}
	}

	#[cfg(feature = "tick")]
	fn tick(&mut self, tick: rx_bevy_core::Tick) {
		if !self.is_closed()
			&& let Ok(mut multicast) = self.multicast.write()
		{
			for (_, destination) in multicast.slab.iter_mut() {
				destination.tick(tick.clone());
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
		for mut destination in self.close_and_drain() {
			destination.unsubscribe();
		}
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		if let Ok(mut multicast) = self.multicast.write() {
			multicast.add(subscription);
		}
	}
}

impl<T, Error> Drop for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	// Must not unsubscribe on drop, it's the shared destination that should do that
	fn drop(&mut self) {}
}
