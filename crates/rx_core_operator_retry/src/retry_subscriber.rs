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
	source: Arc<Mutex<Option<Source>>>,
	retries: usize,
	max_retries: usize,
	finished: bool,
	#[destination]
	destination: SharedSubscriber<Destination>,
	outer_subscription: SubscriptionHandle,
	last_subscription: Arc<Mutex<Option<SubscriptionHandle>>>,
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
		last_subscription: Arc<Mutex<Option<SubscriptionHandle>>>,
		caught_error: Arc<Mutex<Option<Source::OutError>>>,
	) -> Self {
		Self {
			source,
			destination,
			outer_subscription,
			finished: false,
			max_retries,
			retries,
			caught_error,
			last_subscription,
		}
	}

	#[inline]
	fn unsubscribe_inner_subscription(&mut self) {
		if let Some(mut last_subscription) = self.last_subscription.lock_ignore_poison().take()
			&& !last_subscription.is_closed()
		{
			last_subscription.unsubscribe();
		};
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

			self.unsubscribe_inner_subscription();

			let mut stolen_source = self.source.lock_ignore_poison().take().unwrap();

			self.retries += 1;

			let next_subscription = stolen_source.subscribe(RetrySubscriber::new(
				self.source.clone(),
				self.destination.clone(),
				self.max_retries,
				self.retries,
				self.outer_subscription.clone(),
				self.last_subscription.clone(),
				self.caught_error.clone(),
			));

			self.source.lock_ignore_poison().replace(stolen_source);

			if !next_subscription.is_closed() {
				let mut handle = SubscriptionHandle::default();
				handle.add(next_subscription);
				self.last_subscription
					.lock_ignore_poison()
					.replace(handle.clone());
			}

			if self.caught_error.lock_ignore_poison().is_some() {
				self.unsubscribe_inner_subscription();
				continue;
			} else if self.is_closed() {
				self.unsubscribe_inner_subscription();
				break;
			} else {
				self.outer_subscription.add(self.last_subscription.clone());
				break;
			}
		}

		if self.retries > self.max_retries {
			self.finished = true;
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.finished = true;
		self.destination.complete();
		self.unsubscribe();
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
		if self.finished || self.retries > self.max_retries {
			self.unsubscribe_inner_subscription();
			self.destination.unsubscribe();
			self.outer_subscription.unsubscribe();
		}
	}
}
