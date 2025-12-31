use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, SharedSubscriber, Subscriber, SubscriptionHandle,
	SubscriptionLike, TeardownCollectionExtension,
};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
pub struct RetrySubscriber<Source, Destination>
where
	Source: 'static + Observable + Send + Sync,
	Destination: 'static + Subscriber<In = Source::Out, InError = Source::OutError>,
{
	// source: ErasedObservable<Destination::In, Destination::InError>,
	source: Arc<Mutex<Option<Source>>>,
	retries: usize,
	max_retries: usize,
	#[destination]
	destination: SharedSubscriber<Destination>,
	outer_subscription: SubscriptionHandle,
	caught_error: Arc<Mutex<Option<Source::OutError>>>,
}

impl<Source, Destination> RetrySubscriber<Source, Destination>
where
	Source: 'static + Observable + Send + Sync,
	Destination: 'static + Subscriber<In = Source::Out, InError = Source::OutError>,
{
	pub(crate) fn new(
		source: Arc<Mutex<Option<Source>>>,
		destination: SharedSubscriber<Destination>,
		max_retries: usize,
		retries: usize,
		outer_subscription: SubscriptionHandle,
		caught_error: Arc<Mutex<Option<Source::OutError>>>,
	) -> Self {
		Self {
			source,
			destination,
			outer_subscription,
			max_retries,
			retries,
			caught_error,
		}
	}
}

impl<Source, Destination> Observer for RetrySubscriber<Source, Destination>
where
	Source: 'static + Observable + Send + Sync,
	Destination: 'static + Subscriber<In = Source::Out, InError = Source::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		// If the source is "stolen", it's still owned by the observables
		// subscribe method. it will check the caught error and retry there.
		if self.source.lock_ignore_poison().is_none() {
			self.caught_error.lock_ignore_poison().replace(error);
			return;
		};

		while self.retries <= self.max_retries {
			self.caught_error.lock_ignore_poison().take();

			let mut stolen_source = self.source.lock_ignore_poison().take().unwrap();
			let next_subscription = stolen_source.subscribe(RetrySubscriber::new(
				self.source.clone(),
				self.destination.clone(),
				self.max_retries,
				self.retries,
				self.outer_subscription.clone(),
				self.caught_error.clone(),
			));
			self.source.lock_ignore_poison().replace(stolen_source);
			self.retries += 1;
			if !next_subscription.is_closed() {
				self.outer_subscription.add(next_subscription);
				break;
			}

			self.outer_subscription.add(next_subscription);
		}

		if self.retries > self.max_retries {
			self.destination.error(error);
			self.destination.unsubscribe();
			self.outer_subscription.unsubscribe();
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<Source, Destination> SubscriptionLike for RetrySubscriber<Source, Destination>
where
	Source: 'static + Observable + Send + Sync,
	Destination: 'static + Subscriber<In = Source::Out, InError = Source::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		if self.retries > self.max_retries {
			self.destination.unsubscribe();
			self.outer_subscription.unsubscribe();
		}
	}
}
