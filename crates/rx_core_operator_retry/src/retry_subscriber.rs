use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, ObserverTerminalNotification, SharedSubscriber,
	SharedSubscription, Subscriber, SubscriptionLike, TeardownCollectionExtension,
};

pub(crate) const SOURCE_STEAL: &str = "Source should be present!";

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
	finished_with: Option<ObserverTerminalNotification<Source::OutError>>,
	#[destination]
	destination: SharedSubscriber<Destination>,
	outer_subscription: SharedSubscription,
	last_subscription: Arc<Mutex<Option<SharedSubscription>>>,
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
		outer_subscription: SharedSubscription,
		last_subscription: Arc<Mutex<Option<SharedSubscription>>>,
		caught_error: Arc<Mutex<Option<Source::OutError>>>,
	) -> Self {
		Self {
			source,
			destination,
			outer_subscription,
			finished_with: None,
			max_retries,
			retries,
			caught_error,
			last_subscription,
		}
	}

	#[inline]
	fn reset(&mut self) {
		if let Some(mut last_subscription) = self.last_subscription.lock_ignore_poison().take()
			&& !last_subscription.is_closed()
		{
			last_subscription.unsubscribe();
		};

		self.finished_with = None;
		self.caught_error.lock_ignore_poison().take();
	}

	fn finish(&mut self) {
		match self.finished_with.take() {
			Some(ObserverTerminalNotification::Error(error)) => self.destination.error(error),
			Some(ObserverTerminalNotification::Complete) => self.destination.complete(),
			None => {}
		}

		self.outer_subscription.unsubscribe();
		self.destination.unsubscribe();
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

		if self.destination.is_closed() {
			self.unsubscribe();
		}
	}

	fn error(&mut self, error: Self::InError) {
		// If the source is "stolen", it's still owned by the observables
		// subscribe method. It will check the caught error and retry there.
		if self.source.lock_ignore_poison().is_none() {
			self.caught_error.lock_ignore_poison().replace(error);
			return;
		};

		while self.retries <= self.max_retries {
			let mut stolen_source = self.source.lock_ignore_poison().take().expect(SOURCE_STEAL);
			self.reset();
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
				self.last_subscription
					.lock_ignore_poison()
					.replace(SharedSubscription::new(next_subscription));
			}

			if self.caught_error.lock_ignore_poison().is_some() {
				continue;
			} else {
				self.outer_subscription.add(self.last_subscription.clone());
				break;
			}
		}

		self.finished_with = Some(ObserverTerminalNotification::Error(error));

		if self.retries > self.max_retries {
			self.finish();
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.finished_with = Some(ObserverTerminalNotification::Complete);
		self.finish();
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

	fn unsubscribe(&mut self) {
		if self.caught_error.lock_ignore_poison().is_none() && self.finished_with.is_none() {
			self.finish();
		}
	}
}
