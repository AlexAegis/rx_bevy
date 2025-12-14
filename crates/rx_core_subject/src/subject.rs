use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	Finishable, Never, Observable, Observer, Signal, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};

use crate::{Multicast, MulticastSubscription};

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
#[derive(RxSubject, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct Subject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub multicast: Arc<RwLock<Multicast<In, InError>>>,
}

impl<In, InError> Finishable for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_finished(&self) -> bool {
		self.multicast
			.read()
			.unwrap_or_else(|poison_error| poison_error.into_inner())
			.is_finished()
	}
}

impl<In, InError> Clone for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<In, InError> Default for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(Multicast::default())),
		}
	}
}

impl<In, InError> Observable for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut multicast = self
			.multicast
			.write()
			.unwrap_or_else(|poison_error| poison_error.into_inner());
		multicast.subscribe(destination)
	}
}

impl<In, InError> Observer for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		self.multicast.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.multicast.error(error);
	}

	fn complete(&mut self) {
		self.multicast.complete();
	}
}

impl<In, InError> SubscriptionLike for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn is_closed(&self) -> bool {
		self.multicast.is_closed()
	}

	fn unsubscribe(&mut self) {
		// It's an unsubscribe, we can ignore the poison
		if let Some(subscribers) = {
			let mut lock = self
				.multicast
				.write()
				.unwrap_or_else(|poison_error| poison_error.into_inner());

			lock.close()
		} {
			for mut destination in subscribers {
				destination.unsubscribe();
			}
		}
	}
}

impl<In, InError> Drop for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}
