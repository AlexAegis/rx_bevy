use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, ObservableOutput, Observer, ObserverInput,
	PrimaryCategorySubject, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike,
	UpgradeableObserver, WithPrimaryCategory,
};
use slab::Slab;
use stealcell::StealCell;

use crate::MulticastSubscription;

#[derive_where(Default)]
struct Subscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
	subscribers: StealCell<Slab<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>>>,
}

impl<In, InError> Subscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn clean(&mut self) {
		self.subscribers
			.as_mut()
			.retain(|_, subscriber| !subscriber.is_closed());
	}

	#[inline]
	pub(crate) fn drain(&mut self) -> Vec<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>> {
		self.subscribers.drain().collect::<Vec<_>>()
	}
}

#[derive_where(Clone, Default)]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
struct SharedSubscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
	subscribers: Arc<Mutex<Subscribers<In, InError>>>,
}

impl<In, InError> SharedSubscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
}

impl<In, InError> Observer for SharedSubscribers<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		// TODO: Maybe do it one by one, making sure it's unlocked when an action is performed into it
		// and if it closes in the meanwhile, don't put it back!
		let mut stolen_subscribers = {
			println!("shared sub, arc lock");
			let mut subscribers = self.subscribers.lock_ignore_poison();
			subscribers.subscribers.steal()
		};
		println!("shared sub, arc lock release");
		for (_key, destination) in stolen_subscribers.iter_mut() {
			destination.next(next.clone());
		}
		// Still locks up

		println!("shared sub, arc lock2");
		let mut subscribers = self.subscribers.lock_ignore_poison();
		subscribers.subscribers.return_stolen(stolen_subscribers);
		subscribers.clean();
		println!("shared sub, arc lock2 release");
	}

	fn error(&mut self, error: Self::InError) {
		{
			let mut subscribers = self.subscribers.lock_ignore_poison();

			for (key, destination) in subscribers.subscribers.iter_mut() {
				destination.error(error.clone());
			}
		}
		self.unsubscribe();
	}

	fn complete(&mut self) {
		{
			let mut subscribers = self.subscribers.lock_ignore_poison();

			for (key, destination) in subscribers.subscribers.iter_mut() {
				destination.complete();
			}
		}
		self.unsubscribe();
	}
}

impl<In, InError> SubscriptionLike for SharedSubscribers<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self) {}
}

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_teardown_collection]
pub struct PluckSubscriber<Destination>
where
	Destination: Subscriber,
{
	id: usize,
	#[destination]
	destination: Destination,
	subscribers: SharedSubscribers<Destination::In, Destination::InError>,
}
impl<Destination> PluckSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn new(
		destination: Destination,
		id: usize,
		subscribers: SharedSubscribers<Destination::In, Destination::InError>,
	) -> Self {
		Self {
			destination,
			id,
			subscribers,
		}
	}
}

impl<Destination> SubscriptionLike for PluckSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		{
			println!("about to remove itself from subslab {}", self.id);
			let mut subscribers = self.subscribers.subscribers.lock_ignore_poison();
			subscribers.subscribers.remove(self.id);
		}
		self.destination.unsubscribe();
	}
}

#[derive_where(Debug)]
pub struct Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[derive_where(skip(Debug))]
	subscribers: SharedSubscribers<In, InError>,
	closed_flag: SubscriptionClosedFlag,
	is_completed: bool,
	#[derive_where(skip(Debug))]
	last_observed_error: Option<InError>,
}

impl<In, InError> Multicast<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Drops all closed subscribers
	fn clean(&mut self) {
		self.subscribers.subscribers.lock_ignore_poison().clean();
	}

	pub fn get_error(&self) -> Option<&InError> {
		self.last_observed_error.as_ref()
	}

	/// Closes the multicast and drains all its resources so the caller
	/// can perform an unsubscribe
	#[inline]
	pub(crate) fn close_and_drain(
		&mut self,
	) -> Vec<Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>> {
		self.closed_flag.close();
		self.subscribers.subscribers.lock_ignore_poison().drain()
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

		if let Some(error) = self.last_observed_error.clone() {
			destination.error(error);
		} else if self.is_completed {
			destination.complete();
		}

		if self.is_closed() {
			destination.unsubscribe();
			MulticastSubscription::new_closed()
		} else {
			let mut subscribers = self.subscribers.subscribers.lock_ignore_poison();
			let entry = subscribers.subscribers.vacant_entry();
			let shared = Arc::new(Mutex::new(PluckSubscriber::new(
				destination,
				entry.key(),
				self.subscribers.clone(),
			)));
			entry.insert(shared.clone());

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
		if !self.is_closed() {
			self.subscribers.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.last_observed_error = Some(error.clone());
			self.subscribers.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.is_completed = true;
			self.subscribers.complete();
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
			for mut destination in self.close_and_drain() {
				destination.unsubscribe();
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
			subscribers: SharedSubscribers::default(),
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
