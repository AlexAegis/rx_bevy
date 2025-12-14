use derive_where::derive_where;

use crate::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Signal, Subscriber,
	SubscriptionLike, Teardown, TeardownCollection, WithPrimaryCategory,
};

// Boxed erased subscriber so it can be owned inside containers like RwLock.
pub type DynSubscriber<In, InError> = Box<dyn Subscriber<In = In, InError = InError>>;

#[derive_where(Debug)]
pub struct ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip(Debug))]
	destination: Box<dyn Subscriber<In = In, InError = InError>>,
}

impl<In, InError> ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new<Destination>(destination: Destination) -> Self
	where
		Destination: 'static + Subscriber<In = In, InError = InError>,
	{
		Self {
			destination: Box::new(destination),
		}
	}
}

impl<In, InError> WithPrimaryCategory for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError> ObserverUpgradesToSelf for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
}

impl<In, InError> ObserverInput for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError> SubscriptionLike for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

impl<In, InError> TeardownCollection for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.destination.add_teardown(teardown);
	}
}

impl<In, InError> Drop for ErasedSubscriber<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.unsubscribe();
		}
	}
}
