use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_traits::{
	Finishable, Observable, ObservableOutput, Observer, ObserverInput, PrimaryCategorySubject,
	Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike, UpgradeableObserver,
	WithPrimaryCategory,
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
	is_completed: bool,
	#[derive_where(skip(Debug))]
	last_observed_error: Option<InError>,
}

impl<In, InError> Finishable for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_finished(&self) -> bool {
		self.is_completed || self.last_observed_error.is_some() || *self.closed_flag
	}
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

	pub fn get_error(&self) -> Option<&InError> {
		self.last_observed_error.as_ref()
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
		let mut destination = destination.upgrade();
		if self.is_finished() {
			if let Some(error) = self.last_observed_error.clone() {
				destination.error(error);
			} else if self.is_completed {
				destination.complete();
			}

			destination.unsubscribe();
			MulticastSubscription::new_closed()
		} else {
			let shared = Arc::new(Mutex::new(destination));
			self.subscribers.push(shared.clone());
			MulticastSubscription::new(shared)
		}
	}
}

impl<In, InError> Observer for Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_finished() {
			for destination in self.subscribers.iter_mut() {
				destination.next(next.clone());
			}
			self.clean();
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_finished() {
			self.last_observed_error = Some(error.clone());
			for destination in self.subscribers.iter_mut() {
				destination.error(error.clone());
			}
		}
	}

	fn complete(&mut self) {
		if !self.is_finished() {
			self.is_completed = true;
			for destination in self.subscribers.iter_mut() {
				destination.complete();
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
			is_completed: false,
			last_observed_error: None,
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
