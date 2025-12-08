use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{
	Never, Observer, Signal, SubscriberNotification, SubscriptionClosedFlag, SubscriptionLike,
	Teardown, TeardownCollection,
};

use crate::notification_collector::TestNotificationCollector;

/// While this is conceptually an Observer, used as an Observer, for testing
/// purposes it behaves like a Subscriber by not being detached on upgrade.
#[derive(RxObserver, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_upgrades_to(self)]
pub struct MockObserver<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	pub closed_flag: SubscriptionClosedFlag,
	notification_collector: Arc<Mutex<TestNotificationCollector<In, InError>>>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(notification_collector: Arc<Mutex<TestNotificationCollector<In, InError>>>) -> Self {
		Self {
			notification_collector,
			closed_flag: SubscriptionClosedFlag::default(),
			_phantom_data: PhantomData,
		}
	}

	fn lock_collector(
		&mut self,
	) -> std::sync::MutexGuard<'_, TestNotificationCollector<In, InError>> {
		self.notification_collector
			.lock()
			.unwrap_or_else(|p| p.into_inner())
	}
}

impl<In, InError> Observer for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: Self::In) {
		self.lock_collector()
			.push(SubscriberNotification::Next(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.lock_collector()
			.push(SubscriberNotification::Error(error));
	}

	fn complete(&mut self) {
		self.lock_collector().push(SubscriberNotification::Complete);
	}
}

impl<In, InError> SubscriptionLike for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		self.closed_flag.close();
		self.lock_collector()
			.push(SubscriberNotification::Unsubscribe);
	}
}

impl<In, InError> TeardownCollection for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		if self.is_closed() {
			teardown.execute();
		}
		self.lock_collector()
			.push(SubscriberNotification::Add(None));
	}
}

impl<In, InError> Default for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn default() -> Self {
		Self {
			closed_flag: false.into(),
			notification_collector: Arc::new(Mutex::new(TestNotificationCollector::default())),
			_phantom_data: PhantomData,
		}
	}
}
