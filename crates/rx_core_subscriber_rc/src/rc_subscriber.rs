use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, SharedDestination, SharedSubscriber, Subscriber, SubscriptionData, SubscriptionLike,
	Teardown, TeardownCollection,
};

use crate::{InnerRcSubscriber, WeakRcSubscriber};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
pub struct RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	shared_destination: SharedSubscriber<InnerRcSubscriber<Destination>>,
	pub inner_teardown: Option<SubscriptionData>,
	completed: bool,
	unsubscribed: bool,
}

impl<Destination> Clone for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn clone(&self) -> Self {
		let mut shared_destination = self.shared_destination.clone();

		shared_destination.access_mut(|destination| {
			destination.ref_count += 1;

			if self.completed {
				destination.completion_count += 1;
			}

			if self.unsubscribed {
				destination.unsubscribe_count += 1;
			}
		});

		Self {
			completed: self.completed,
			unsubscribed: self.unsubscribed,
			inner_teardown: None,
			shared_destination,
		}
	}
}

impl<Destination> RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			shared_destination: SharedSubscriber::new(InnerRcSubscriber::new(destination)),
			inner_teardown: None,
			completed: false,
			unsubscribed: false,
		}
	}

	#[inline]
	pub fn is_this_clone_closed(&self) -> bool {
		self.unsubscribed || self.completed
	}

	#[inline]
	pub fn add_downstream_teardown(&mut self, teardown: Teardown) {
		self.shared_destination.add_teardown(teardown);
	}

	/// Acquire a clone to the same reference which will not interact with
	/// the reference counts, and only attempts to complete or unsubscribe it
	/// when it too completes or unsubscribes. And can still be used as a
	/// subscriber
	pub fn downgrade(&self) -> WeakRcSubscriber<Destination> {
		WeakRcSubscriber {
			shared_destination: self.shared_destination.clone(),
		}
	}
}

impl<Destination> Observer for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In) {
		if !self.is_this_clone_closed() {
			self.shared_destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_this_clone_closed() {
			self.shared_destination.error(error);
			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_this_clone_closed() {
			self.completed = true;
			self.shared_destination.access_mut(|destination| {
				destination.completion_count += 1;
				destination.complete_if_can();
			});
			self.shared_destination.complete();
		}
	}
}

impl<Destination> SubscriptionLike for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.shared_destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.unsubscribed {
			self.unsubscribed = true;
			self.shared_destination.access_mut(|destination| {
				destination.unsubscribe_count += 1;
			});
			self.shared_destination.unsubscribe();
		}
	}
}

impl<Destination> TeardownCollection for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		// The inner subscriptions additional teardowns will be stored here, not downstream.
		// Additional downstream teardowns can only be added from upstream, using an externally
		// accessed function.
		self.inner_teardown
			.get_or_insert_default()
			.add_teardown(teardown);
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		let completed = self.completed;
		let unsubscribed = self.unsubscribed;

		self.access_mut(|inner_destination| {
			inner_destination.ref_count -= 1;

			if completed {
				inner_destination.completion_count -= 1;
			}

			if unsubscribed {
				inner_destination.unsubscribe_count -= 1;
			}
		});
	}
}

impl<Destination> SharedDestination<InnerRcSubscriber<Destination>> for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access(accessor);
	}

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access_mut(accessor);
	}
}

#[cfg(test)]
mod test {
	use rx_core::prelude::*;
	use rx_core_testing::{MockObserver, SharedNotificationCollector};
	use rx_core_traits::{SharedDestination, SubscriptionLike};

	use crate::RcSubscriber;

	fn setup() -> (
		RcSubscriber<MockObserver<i32>>,
		SharedNotificationCollector<i32>,
	) {
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();
		let rc_subscriber = RcSubscriber::new(mock_destination);

		(rc_subscriber, notification_collector)
	}

	#[test]
	fn rc_subscriber_starts_with_ref_1() {
		let (mut rc_subscriber, _notification_collector) = setup();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe();
	}

	#[test]
	fn rc_subscriber_unsubscribes() {
		let (mut rc_subscriber, notification_collector) = setup();

		Observer::next(&mut rc_subscriber, 1);
		rc_subscriber.unsubscribe();

		assert_eq!(
			notification_collector
				.lock()
				.count_all_observed_unsubscribes(),
			1
		);
	}

	#[test]
	fn rc_subscriber_clone_unsubscribing_should_not_unsubscribe_destination() {
		let (mut rc_subscriber, notification_collector) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		assert_eq!(
			notification_collector
				.lock()
				.count_all_observed_unsubscribes(),
			0
		);

		rc_subscriber_clone.unsubscribe();
		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		assert_eq!(
			notification_collector
				.lock()
				.count_all_observed_unsubscribes(),
			0
		);

		rc_subscriber.unsubscribe();
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe() {
		let (mut rc_subscriber, notification_collector) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed()
		);
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe_drop_does_not_remove_ref_count() {
		let (mut rc_subscriber, notification_collector) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_subscriber_clone);

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber.unsubscribe();
		// A debug assertion in the `Drop` of [InnerRcSubscriber] asserts that
		// `ref_count` and `unsubscribe_count` are equal.
		drop(rc_subscriber);

		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed()
		);
	}

	#[test]
	fn rc_subscriber_as_iterator_observable_target_direct() {
		let (mut rc_subscriber, _notification_collector) = setup();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a = IteratorObservable::new(1..=10);

		let mut iterator_a_subscription = iterator_a.subscribe(rc_subscriber);

		iterator_a_subscription.unsubscribe();
	}

	#[test]
	fn rc_subscriber_as_iterator_observable_target_cloned() {
		let (mut rc_subscriber, _notification_collector) = setup();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a = IteratorObservable::new(1..=10);

		let iterator_a_destination = rc_subscriber.clone();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a_subscription = iterator_a.subscribe(iterator_a_destination);

		// The iterator immediately completes and unsubscribes.
		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		// Additional unsubscribe calls and letting the clone drop does not
		// increase the counter any further
		iterator_a_subscription.unsubscribe();
		drop(iterator_a_subscription);

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		drop(rc_subscriber);
	}

	#[test]
	fn rc_subscriber_triple_clone_ref_count() {
		let (mut rc_subscriber, _notification_collector) = setup();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut rc_clone_1 = rc_subscriber.clone();
		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		rc_clone_1.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		let mut rc_clone_2 = rc_subscriber.clone();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_clone_2.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_clone_1);

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		drop(rc_clone_2);

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe();

		rc_subscriber.access(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		drop(rc_subscriber);
	}
}
