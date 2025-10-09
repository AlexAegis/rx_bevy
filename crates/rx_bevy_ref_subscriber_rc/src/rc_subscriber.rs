use rx_bevy_core::{
	ArcSubscriber, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick,
	WithContext,
};

use crate::{InnerRcSubscriber, WeakRcSubscriber};

pub struct RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	// TODO: Use a generic SHARER, Instead of an Arc, all this should guarantee that the destination is cloneable and it still points to the same thing. This is true for entities aswell
	destination: ArcSubscriber<InnerRcSubscriber<Destination>>,
	completed: bool,
	unsubscribed: bool,
}

impl<Destination> From<Destination> for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn from(destination: Destination) -> Self {
		Self::new(destination)
	}
}

impl<Destination> RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: ArcSubscriber::new(InnerRcSubscriber::new(destination)),
			completed: false,
			unsubscribed: false,
		}
	}

	#[inline]
	pub fn is_this_clone_closed(&self) -> bool {
		self.unsubscribed || self.completed
	}

	/// Let's you check the shared observer for the duration of the callback
	#[inline]
	pub fn read<F>(&mut self, reader: F)
	where
		F: Fn(&InnerRcSubscriber<Destination>),
	{
		self.destination.read(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	#[inline]
	pub fn write<F>(&mut self, writer: F)
	where
		F: FnMut(&mut InnerRcSubscriber<Destination>),
	{
		self.destination.write(writer);
	}

	/// Acquire a clone to the same reference which will not interact with
	/// the reference counts, and only attempts to complete or unsubscribe it
	/// when it too completes or unsubscribes. And can still be used as a
	/// subscriber
	pub fn downgrade(&self) -> WeakRcSubscriber<Destination> {
		WeakRcSubscriber {
			destination: self.destination.clone(),
			closed: self.completed || self.unsubscribed,
		}
	}
}

impl<Destination> Clone for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn clone(&self) -> Self {
		self.destination.write(|destination| {
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
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithContext for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	type Context = Destination::Context;
}

impl<Destination> Observer for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.destination.next(next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.completed = true;
			self.destination.write(|destination| {
				destination.completion_count += 1;
			});
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_this_clone_closed() {
			self.destination.tick(tick, context);
		}
	}
}

impl<Destination> SubscriptionLike for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.unsubscribed {
			self.unsubscribed = true;
			self.destination.write(|destination| {
				destination.unsubscribe_count += 1;
			});
			self.destination.unsubscribe(context);
		}
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Destination> Drop for RcSubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		if !self.unsubscribed {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
		self.destination.write(|destination| {
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
	use rx_bevy_core::{DropSafeSignalContext, Observer, SubscriptionLike};
	use rx_bevy_observable_iterator::IteratorObservable;
	use rx_bevy_testing::{MockContext, MockObserver};

	use crate::RcSubscriber;

	fn setup() -> (
		RcSubscriber<MockObserver<i32, (), DropSafeSignalContext>>,
		MockContext<i32, (), DropSafeSignalContext>,
	) {
		let context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSignalContext>::default();

		let rc_subscriber = RcSubscriber::new(mock_destination);

		(rc_subscriber, context)
	}

	#[test]
	fn rc_subscriber_starts_with_ref_1() {
		let (mut rc_subscriber, mut _context) = setup();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
	}

	#[test]
	fn rc_subscriber_unsubscribes() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.next(1, &mut context);
		rc_subscriber.unsubscribe(&mut context);

		assert_eq!(context.all_unsubscribes(), 1);
	}

	#[test]
	fn rc_subscriber_clone_unsubscribing_should_not_unsubscribe_destination() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		assert_eq!(context.unsubscribes, 0);

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});
		assert_eq!(context.unsubscribes, 0);
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		assert!(context.nothing_happened_after_closed());
	}

	#[test]
	fn rc_subscriber_clones_unsubscribe_drop_removes_ref_count() {
		let (mut rc_subscriber, mut context) = setup();
		let mut rc_subscriber_clone = rc_subscriber.clone();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		rc_subscriber.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		rc_subscriber_clone.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_subscriber_clone);

		rc_subscriber.destination.read(|destination| {
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

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a = IteratorObservable::<
			RangeInclusive<i32>,
			MockContext<i32, (), DropSafeSignalContext>,
		>::new(1..=10);

		let mut iterator_a_subscription = iterator_a.subscribe(rc_subscriber, &mut context);

		iterator_a_subscription.unsubscribe(&mut context);
	}

	#[test]
	fn rc_subscriber_as_iterator_observable_target_cloned() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut iterator_a = IteratorObservable::<
			RangeInclusive<i32>,
			MockContext<i32, (), DropSafeSignalContext>,
		>::new(1..=10);

		let mut iterator_a_subscription = iterator_a.subscribe(rc_subscriber.clone(), &mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		// Let the clone drop
		iterator_a_subscription.unsubscribe(&mut context);

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
	}

	#[test]
	fn rc_subscriber_asd() {
		let (mut rc_subscriber, mut context) = setup();

		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});

		let mut rc_clone_1 = rc_subscriber.clone();
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		rc_clone_1.unsubscribe(&mut context);
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		let mut rc_clone_2 = rc_subscriber.clone();
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 1);
		});
		rc_clone_2.unsubscribe(&mut context);
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 3);
			assert_eq!(destination.unsubscribe_count, 2);
		});

		drop(rc_clone_1);
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 2);
			assert_eq!(destination.unsubscribe_count, 1);
		});

		drop(rc_clone_2);
		rc_subscriber.destination.read(|destination| {
			assert_eq!(destination.ref_count, 1);
			assert_eq!(destination.unsubscribe_count, 0);
		});
		drop(rc_subscriber);
	}
}
