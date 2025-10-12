use rx_bevy_core::{
	Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick, Tickable,
	context::{
		WithSubscriptionContext,
		allocator::{DestinationAllocator, DestinationSharedTypes, SharedDestination},
	},
};

use crate::{InnerRcSubscriber, WeakRcSubscriber};

/// TODO: move this back to core, it's no longer specialized
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
	pub fn new(destination: Destination, context: &mut Destination::Context) -> Self {
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

	pub fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access(accessor);
	}

	pub fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>),
	{
		self.shared_destination.access_mut(accessor);
	}

	pub fn access_with_context<F>(&mut self, accessor: F, context: &mut Destination::Context)
	where
		F: Fn(&InnerRcSubscriber<Destination>, &mut Destination::Context),
	{
		self.shared_destination
			.access_with_context(accessor, context);
	}

	pub fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut Destination::Context)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>, &mut Destination::Context),
	{
		self.shared_destination
			.access_with_context_mut(accessor, context);
	}

	/// Acquire a clone to the same reference which will not interact with
	/// the reference counts, and only attempts to complete or unsubscribe it
	/// when it too completes or unsubscribes. And can still be used as a
	/// subscriber
	pub fn downgrade(&self) -> WeakRcSubscriber<Destination> {
		WeakRcSubscriber {
			shared_destination: self.shared_destination.clone(),
			closed: self.completed || self.unsubscribed,
		}
	}
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
			shared_destination,
		}
	}
}

impl<Destination> ObserverInput for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.shared_destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.shared_destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.completed = true;
			self.shared_destination.access_mut(|destination| {
				destination.completion_count += 1;
			});
			self.shared_destination.complete(context);
			self.unsubscribe(context);
		}
	}
}

impl<Destination> Tickable for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
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

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.unsubscribed {
			self.unsubscribed = true;
			self.shared_destination.access_mut(|destination| {
				destination.unsubscribe_count += 1;
			});
			self.shared_destination.unsubscribe(context);
		}
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.shared_destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.shared_destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		if !self.unsubscribed {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
		self.shared_destination.access_mut(|destination| {
			destination.ref_count -= 1;

			if self.completed {
				destination.completion_count -= 1;
			}

			if self.unsubscribed {
				destination.unsubscribe_count -= 1;
			}
		});
	}
}

#[cfg(test)]
mod test {
	use std::ops::RangeInclusive;

	use rx_bevy_core::Observable;
	use rx_bevy_core::{Observer, SubscriptionLike, context::DropSafeSubscriptionContext};
	use rx_bevy_observable_iterator::IteratorObservable;
	use rx_bevy_testing::{MockContext, MockObserver};

	use crate::RcSubscriber;

	fn setup() -> (
		RcSubscriber<MockObserver<i32, (), DropSafeSubscriptionContext>>,
		MockContext<i32, (), DropSafeSubscriptionContext>,
	) {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

		let rc_subscriber = RcSubscriber::new(mock_destination, &mut context);

		(rc_subscriber, context)
	}

	#[test]
	fn rc_subscriber_starts_with_ref_1() {
		let (mut rc_subscriber, mut _context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
	}

	#[test]
	fn rc_subscriber_unsubscribes() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.next(1, &mut context);
		rc_subscriber.unsubscribe(&mut context);

		assert_eq!(context.count_all_observed_unsubscribes(), 1);
	}

	#[test]
	fn rc_subscriber_clone_unsubscribing_should_not_unsubscribe_destination() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

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
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

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
	fn rc_subscriber_clones_unsubscribe_drop_removes_ref_count() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

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
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		// A debug assertion in the `Drop` of [InnerRcSubscriber] asserts that
		// `ref_count`, `completion_count` and `unsubscribe_count` did indeed
		// drop to 0
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

		let mut iterator_a = IteratorObservable::<
			RangeInclusive<i32>,
			MockContext<i32, (), DropSafeSubscriptionContext>,
		>::new(1..=10);

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

		let mut iterator_a = IteratorObservable::<
			RangeInclusive<i32>,
			MockContext<i32, (), DropSafeSubscriptionContext>,
		>::new(1..=10);

		let mut iterator_a_subscription = iterator_a.subscribe(rc_subscriber.clone(), &mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		// Let the clone drop
		iterator_a_subscription.unsubscribe(&mut context);

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
	}

	#[test]
	fn rc_subscriber_asd() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut rc_clone_1 = rc_subscriber.clone();
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		rc_clone_1.unsubscribe(&mut context);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		let mut rc_clone_2 = rc_subscriber.clone();
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
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		drop(rc_clone_2);
		rc_subscriber.shared_destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		drop(rc_subscriber);
	}
}
