use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{
	Never, Observer, Signal, SubscriberNotification, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection,
};

use crate::NotificationCollector;

/// While this is conceptually an Observer, used as an Observer, for testing
/// purposes it behaves like a Subscriber by not being detached on upgrade.
#[derive_where(Default, Clone)]
#[derive(RxObserver, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_upgrades_to(self)]
pub struct MockObserver<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip(Clone))]
	teardown: SubscriptionData,
	notification_collector: NotificationCollector<In, InError>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(notification_collector: NotificationCollector<In, InError>) -> Self {
		Self {
			notification_collector,
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}

	pub fn get_notification_collector(&self) -> NotificationCollector<In, InError> {
		self.notification_collector.clone()
	}
}

impl<In, InError> Observer for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: Self::In) {
		self.notification_collector
			.lock()
			.push(SubscriberNotification::Next(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.notification_collector
			.lock()
			.push(SubscriberNotification::Error(error));
	}

	fn complete(&mut self) {
		self.notification_collector
			.lock()
			.push(SubscriberNotification::Complete);
	}
}

impl<In, InError> SubscriptionLike for MockObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.teardown.unsubscribe();
		self.notification_collector
			.lock()
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
		self.teardown.add_teardown(teardown);
	}
}
