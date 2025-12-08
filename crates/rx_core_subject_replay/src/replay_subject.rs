use std::sync::{Arc, RwLock};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{Never, Observable, Observer, Signal, Subscriber, UpgradeableObserver};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_delegate_subscription_like_to_destination]
pub struct ReplaySubject<const CAPACITY: usize, In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[destination]
	subject: Subject<In, InError>,
	/// Shared data across clones
	values: Arc<RwLock<ConstGenericRingBuffer<In, CAPACITY>>>,
}

impl<const CAPACITY: usize, In, InError> ReplaySubject<CAPACITY, In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription though to keep your code reactive, only use this when it's
	/// absolutely necessary.
	///
	/// This has a max length of `CAPACITY` but can be empty too, right when
	/// the subject is created and no values have been observed.
	pub fn values(&self) -> Vec<In> {
		self.values
			.read()
			.unwrap_or_else(|poison_error| {
				self.values.clear_poison();
				poison_error.into_inner()
			})
			.iter()
			.cloned()
			.collect()
	}
}

impl<const CAPACITY: usize, In, InError> Default for ReplaySubject<CAPACITY, In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			subject: Subject::default(),
			values: Arc::new(RwLock::new(ConstGenericRingBuffer::default())),
		}
	}
}

impl<const CAPACITY: usize, In, InError> Observer for ReplaySubject<CAPACITY, In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		self.values
			.write()
			.unwrap_or_else(|poison_error| {
				self.values.clear_poison();
				poison_error.into_inner()
			})
			.enqueue(next.clone());

		self.subject.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}
}

impl<const CAPACITY: usize, In, InError> Observable for ReplaySubject<CAPACITY, In, InError>
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
		let mut downstream_subscriber = destination.upgrade();
		let buffer_iter = self
			.values
			.read()
			.unwrap_or_else(|poison_error| {
				self.values.clear_poison();
				poison_error.into_inner()
			})
			.clone()
			.into_iter();

		for value in buffer_iter {
			downstream_subscriber.next(value);
		}

		self.subject.subscribe(downstream_subscriber)
	}
}
