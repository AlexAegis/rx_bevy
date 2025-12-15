use std::sync::{Arc, Mutex};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	LockWithPoisonBehavior, Never, Observable, Observer, Signal, Subscriber, SubscriptionLike,
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
pub struct PublishSubject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	multicast: Arc<Mutex<Multicast<In, InError>>>,
}

impl<In, InError> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	pub fn is_errored(&self) -> bool {
		self.multicast.lock_ignore_poison().get_error().is_some()
	}
}

impl<In, InError> Clone for PublishSubject<In, InError>
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

impl<In, InError> Default for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn default() -> Self {
		Self {
			multicast: Arc::new(Mutex::new(Multicast::default())),
		}
	}
}

impl<In, InError> Observable for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		self.multicast.lock_ignore_poison().subscribe(destination)
	}
}

impl<In, InError> Observer for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.multicast.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.multicast.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.multicast.complete();
	}
}

impl<In, InError> SubscriptionLike for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.multicast.is_closed()
	}

	fn unsubscribe(&mut self) {
		for mut destination in self.multicast.lock_ignore_poison().close_and_drain() {
			destination.unsubscribe();
		}
	}
}

impl<In, InError> Drop for PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}
