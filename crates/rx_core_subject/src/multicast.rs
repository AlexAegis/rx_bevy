use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_traits::{
	Observable, ObservableOutput, Observer, ObserverInput, PrimaryCategorySubject, Signal,
	Subscriber, SubscriptionClosedFlag, SubscriptionLike, UpgradeableObserver, WithPrimaryCategory,
};
use smallvec::SmallVec;

use crate::MulticastSubscription;

#[derive_where(Debug)]
pub struct Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[derive_where(skip(Debug))]
	subscribers: SmallVec<[Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>; 1]>,
	closed_flag: SubscriptionClosedFlag,
}

impl<In, InError> Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Drops all closed subscribers
	fn clean(&mut self) {
		self.subscribers
			.retain(|subscriber| !subscriber.is_closed());
	}

	/// Closes the multicast and drains all its resources so the caller
	/// can perform an unsubscribe
	#[inline]
	pub(crate) fn close(
		&mut self,
	) -> Option<Vec<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>> {
		if self.is_closed() {
			None
		} else {
			let subscribers = self.subscribers.drain(..).collect::<Vec<_>>();
			self.closed_flag.close();
			Some(subscribers)
		}
	}
}

impl<In, InError> Observable for Multicast<In, InError>
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
		if !self.is_closed() {
			let shared = Arc::new(Mutex::new(destination.upgrade()));
			self.subscribers.push(shared.clone());
			MulticastSubscription::new(shared)
		} else {
			MulticastSubscription::new_closed()
		}
	}
}

impl<In, InError> Observer for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			for destination in self.subscribers.iter_mut() {
				destination.next(next.clone());
			}
			self.clean();
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			for mut destination in self.subscribers.drain(..) {
				destination.error(error.clone());
				destination.unsubscribe();
			}
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			for mut destination in self.subscribers.drain(..) {
				destination.complete();
				destination.unsubscribe();
			}
		}
	}
}

impl<In, InError> SubscriptionLike for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed_flag.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();

			if let Some(subscribers) = self.close() {
				for mut destination in subscribers {
					destination.unsubscribe();
				}
			}
		}
	}
}

impl<In, InError> ObserverInput for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> WithPrimaryCategory for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type PrimaryCategory = PrimaryCategorySubject;
}

impl<In, InError> Default for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			subscribers: SmallVec::new(),
			closed_flag: false.into(),
		}
	}
}

impl<In, InError> Drop for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// Does not need to unsubscribe on drop as it's just a collection of
		// shared subscribers, the subscription given to the user is what must
		// be unsubscribed, not the multicast.

		// Close the flag regardless to avoid the safety check on drop.
		self.closed_flag.close();
	}
}
