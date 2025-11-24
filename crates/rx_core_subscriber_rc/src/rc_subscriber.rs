use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Subscriber, SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection,
	Tick, Tickable,
	allocator::{DestinationAllocator, DestinationSharedTypes, SharedDestination},
};

use crate::{InnerRcSubscriber, WeakRcSubscriber};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
pub struct RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	shared_destination: <InnerRcSubscriber<Destination> as DestinationSharedTypes>::Shared,
	completed: bool,
	unsubscribed: bool,
}

impl<Destination> RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	pub fn new(
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			shared_destination:
				<InnerRcSubscriber<Destination> as DestinationSharedTypes>::Sharer::share(
					InnerRcSubscriber::new(destination),
					context,
				),
			completed: false,
			unsubscribed: false,
		}
	}

	#[inline]
	pub fn is_this_clone_closed(&self) -> bool {
		self.unsubscribed || self.completed
	}

	pub fn clone_with_context(
		&self,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let mut shared_destination = self.shared_destination.clone_with_context(context);

		shared_destination.access_with_context_mut(
			|destination, _context| {
				destination.ref_count += 1;

				if self.completed {
					destination.completion_count += 1;
				}

				if self.unsubscribed {
					destination.unsubscribe_count += 1;
				}
			},
			context,
		);

		Self {
			completed: self.completed,
			unsubscribed: self.unsubscribed,
			shared_destination,
		}
	}

	pub fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(
			&InnerRcSubscriber<Destination>,
			&mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
		),
	{
		self.shared_destination
			.access_with_context(accessor, context);
	}

	pub fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(
			&mut InnerRcSubscriber<Destination>,
			&mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
		),
	{
		self.shared_destination
			.access_with_context_mut(accessor, context);
	}

	/// Acquire a clone to the same reference which will not interact with
	/// the reference counts, and only attempts to complete or unsubscribe it
	/// when it too completes or unsubscribes. And can still be used as a
	/// subscriber
	pub fn downgrade(
		&self,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> WeakRcSubscriber<Destination> {
		WeakRcSubscriber {
			shared_destination: self.shared_destination.clone_with_context(context),
			closed_flag: (self.completed || self.unsubscribed).into(),
		}
	}
}

impl<Destination> Observer for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_this_clone_closed() {
			self.shared_destination.next(next, context);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_this_clone_closed() {
			self.shared_destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_this_clone_closed() {
			self.completed = true;
			self.shared_destination.access_with_context_mut(
				|destination, context| {
					destination.completion_count += 1;
					destination.complete_if_can(context);
				},
				context,
			);
			self.shared_destination.complete(context);
		}
	}
}

impl<Destination> Tickable for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// TODO: verify how shared destinations behave if all of them are getting ticked, ticks might need an id, and consumers of ticks would probably need to check if a tick had been processed before using them
		//if !self.is_this_clone_closed() {
		self.shared_destination.tick(tick, context);
		//}
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

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.unsubscribed {
			self.unsubscribed = true;
			self.shared_destination.access_with_context_mut(
				|destination, _context| {
					destination.unsubscribe_count += 1;
				},
				context,
			);
			self.shared_destination.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.shared_destination.add_teardown(teardown, context);
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		// Even though this is a shared subscriber, which usually should not do
		// anything on drop, this is reference counted, and it has to make sure
		// the count happened by unsubscribing itself.
		if !self.unsubscribed {
			let mut context = Destination::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

#[cfg(test)]
mod test {
	use std::ops::RangeInclusive;

	use rx_core::prelude::*;
	use rx_core_testing::{MockContext, MockObserver};
	use rx_core_traits::SubscriptionLike;

	use crate::RcSubscriber;

	fn setup() -> (RcSubscriber<MockObserver<i32>>, MockContext<i32>) {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32>::default();

		let rc_subscriber = RcSubscriber::new(mock_destination, &mut context);

		(rc_subscriber, context)
	}

	#[test]
	fn rc_subscriber_starts_with_ref_1() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe(&mut context);
	}

	#[test]
	fn rc_subscriber_unsubscribes() {
		let (mut rc_subscriber, mut context) = setup();

		Observer::next(&mut rc_subscriber, 1, &mut context);
		rc_subscriber.unsubscribe(&mut context);

		assert_eq!(context.count_all_observed_unsubscribes(), 1);
	}

	#[test]
	fn rc_subscriber_clone_unsubscribing_should_not_unsubscribe_destination() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone_with_context(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		assert_eq!(context.count_all_observed_unsubscribes(), 0);

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});
		assert_eq!(context.count_all_observed_unsubscribes(), 0);
		rc_subscriber.unsubscribe(&mut context);
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone_with_context(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		assert!(context.nothing_happened_after_closed());
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe_drop_does_not_remove_ref_count() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone_with_context(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_subscriber_clone);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		rc_subscriber.unsubscribe(&mut context);
		// A debug assertion in the `Drop` of [InnerRcSubscriber] asserts that
		// `ref_count` and `unsubscribe_count` are equal.
		drop(rc_subscriber);

		assert!(context.nothing_happened_after_closed());
	}

	#[test]
	fn rc_subscriber_as_iterator_observable_target_direct() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a =
			IteratorObservable::<RangeInclusive<i32>, MockContext<i32>>::new(1..=10);

		let mut iterator_a_subscription = iterator_a.subscribe(rc_subscriber, &mut context);

		iterator_a_subscription.unsubscribe(&mut context);
	}

	#[test]
	fn rc_subscriber_as_iterator_observable_target_cloned() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a =
			IteratorObservable::<RangeInclusive<i32>, MockContext<i32>>::new(1..=10);

		let iterator_a_destination = rc_subscriber.clone_with_context(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a_subscription =
			iterator_a.subscribe(iterator_a_destination, &mut context);

		// The iterator immediately compeltes and unsubscribes.
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		// Additional unsubscrbe calls and letting the clone drop does not increase the counter any further
		iterator_a_subscription.unsubscribe(&mut context);
		drop(iterator_a_subscription);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});
		rc_subscriber.unsubscribe(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});
		drop(rc_subscriber);
	}

	#[test]
	fn rc_subscriber_triple_clone_ref_count() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut rc_clone_1 = rc_subscriber.clone_with_context(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		rc_clone_1.unsubscribe(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		let mut rc_clone_2 = rc_subscriber.clone_with_context(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 1);
		});
		rc_clone_2.unsubscribe(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_clone_1);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_clone_2);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 2);
		});
		rc_subscriber.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 3);
		});
		drop(rc_subscriber);
	}
}
