use core::marker::PhantomData;

use rx_core_traits::{
	Observable, ObservableOutput, Observer, PrimaryCategoryObservable, SignalBound,
	SubscriptionContext, SubscriptionLike, UpgradeableObserver, WithPrimaryCategory,
	WithSubscriptionContext,
};

use crate::{OnTickIteratorSubscription, observable::OnTickObservableOptions};

/// Emits an iterators values one at a time at every nth tick, regardless how
/// long each tick was. Mostly meant for debugging purposes, or just to observe
/// `n` amount of steady ticks of the scheduler used.
///
/// > Warning! This is not the same thing as creating a timer, for that use
/// > the [rx_core_observable_interval::IntervalObservable]!
///
/// An example usecase is throttling a logger to every nth frame, where knowing
/// exactly how many frames have passed is useful. Otherwise, the
/// IntervalObservable is a better choice for throttling.
#[derive(Clone, Debug)]
pub struct IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SubscriptionContext,
{
	iterator: Iterator,
	options: OnTickObservableOptions,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Iterator, Context> IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SubscriptionContext,
{
	pub fn new(iterator: Iterator, options: OnTickObservableOptions) -> Self {
		Self {
			iterator,
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<Iterator, Context> ObservableOutput for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> WithSubscriptionContext for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Iterator, Context> WithPrimaryCategory for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<Iterator, Context> Observable for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Iterator::IntoIter: Send + Sync,
	Context: SubscriptionContext,
{
	type Subscription = OnTickIteratorSubscription<Iterator, Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut destination = observer.upgrade();
		let mut iter = self.iterator.clone().into_iter();
		if self.options.emit_at_every_nth_tick == 0 {
			let mut completed = true;
			for item in iter.by_ref() {
				if destination.is_closed() {
					completed = false;
					break;
				}
				destination.next(item, context);
			}
			if completed {
				destination.complete(context);
			}
		}
		OnTickIteratorSubscription::new(destination, iter, self.options.clone(), context)
	}
}

#[cfg(test)]
mod test_iterator_on_tick_observable {

	mod when_emit_at_nth_is_non_zero {

		use std::time::Duration;

		use rx_core::prelude::*;
		use rx_core_testing::prelude::*;
		use rx_core_traits::SubscriberNotification;

		use crate::observable::{IntoIteratorOnTickObservableExtension, OnTickObservableOptions};

		/// Verifies:
		/// - RX_OB_IMMEDIATE_COMPLETION
		/// - RX_OB_UNSUBSCRIBE_AFTER_COMPLETE
		/// - RX_NO_MORE_NOTIFICATIONS_AFTER_CLOSE_EXCEPT_TICKS
		/// - RX_ALWAYS_FORWARD_TICKS
		#[test]
		fn should_emit_its_values_every_two_ticks_then_complete() {
			let mut mock_clock = MockClock::default();
			let mut context = MockContext::default();
			let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

			let mut source = (1..=3).into_observable_on_every_nth_tick::<MockContext<_, _, _>>(
				OnTickObservableOptions {
					emit_at_every_nth_tick: 2,
					start_on_subscribe: true,
				},
			);
			let mut subscription = source.subscribe(mock_destination, &mut context);
			assert!(matches!(
				context.nth_notification(0),
				SubscriberNotification::Next(1)
			));
			subscription.tick(mock_clock.elapse(Duration::from_millis(1)), &mut context);
			assert!(matches!(
				context.nth_notification(1),
				SubscriberNotification::Tick(_)
			));
			assert!(
				!context.nth_notification_exists(2),
				"should not have emitted after only one tick"
			);
			subscription.tick(mock_clock.elapse(Duration::from_millis(3)), &mut context);
			assert!(matches!(
				context.nth_notification(2),
				SubscriberNotification::Next(2)
			));
			assert!(matches!(
				context.nth_notification(3),
				SubscriberNotification::Tick(_)
			));

			subscription.tick(mock_clock.elapse(Duration::from_millis(2)), &mut context);
			assert!(matches!(
				context.nth_notification(4),
				SubscriberNotification::Tick(_)
			));
			subscription.tick(mock_clock.elapse(Duration::from_millis(1)), &mut context);
			assert!(matches!(
				context.nth_notification(5),
				SubscriberNotification::Next(3)
			));
			assert!(matches!(
				context.nth_notification(6),
				SubscriberNotification::Complete
			));
			assert!(matches!(
				context.nth_notification(7),
				SubscriberNotification::Unsubscribe
			));
			assert!(matches!(
				context.nth_notification(8),
				SubscriberNotification::Tick(_)
			));

			assert_eq!(context.all_observed_values(), vec![1, 2, 3]);

			subscription.unsubscribe(&mut context);
			println!("{:?}", context);
			assert!(!context.nth_notification_exists(9));
			assert!(
				context.nothing_happened_after_closed(),
				"something happened after unsubscribe"
			);
		}
	}

	mod when_emit_at_nth_is_zero {

		use std::time::Duration;

		use rx_core::prelude::*;
		use rx_core_testing::{IteratorTrackingDataAccess, TrackedIterator, prelude::*};
		use rx_core_traits::SubscriberNotification;

		use crate::observable::{IntoIteratorOnTickObservableExtension, OnTickObservableOptions};

		#[test]
		fn should_immediately_emit_all_its_values_then_complete() {
			let mut mock_clock = MockClock::default();
			let mut context = MockContext::default();
			let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

			let mut source = (1..=3).into_observable_on_every_nth_tick::<MockContext<_, _, _>>(
				OnTickObservableOptions {
					emit_at_every_nth_tick: 0,
					start_on_subscribe: false,
				},
			);
			let mut subscription = source.subscribe(mock_destination, &mut context);
			println!("{:?}", context);
			assert_eq!(context.all_observed_values(), vec![1, 2, 3]);
			assert!(matches!(
				context.nth_notification(3),
				SubscriberNotification::Complete
			));
			subscription.tick(mock_clock.elapse(Duration::from_millis(1)), &mut context);
			assert!(matches!(
				context.nth_notification(4),
				SubscriberNotification::Tick(_)
			));
			assert!(
				!context.nth_notification_exists(5),
				"Something happened after completion due to a tick!"
			);
			subscription.unsubscribe(&mut context);
			assert!(
				context.nothing_happened_after_closed(),
				"something happened after unsubscribe"
			);
		}

		/// Verifies:
		/// - RX_CHECK_CLOSED_ON_MULTI_EMISSIONS
		#[test]
		fn should_not_finish_the_iterator_when_closed_early() {
			let mut context = MockContext::default();
			let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

			let tracked_iterator = TrackedIterator::new(1..=5);
			let tracked_data = tracked_iterator.get_tracking_data_ref();
			let mut source = tracked_iterator
				.into_observable_on_every_nth_tick::<MockContext<_, _, _>>(
					OnTickObservableOptions {
						emit_at_every_nth_tick: 0,
						start_on_subscribe: false,
					},
				)
				.take(2);
			let mut subscription = source.subscribe(mock_destination, &mut context);
			println!("{:?}", context);
			assert!(matches!(
				context.nth_notification(0),
				SubscriberNotification::Next(1)
			));
			assert!(matches!(
				context.nth_notification(1),
				SubscriberNotification::Next(2)
			));
			assert!(matches!(
				context.nth_notification(2),
				SubscriberNotification::Complete
			));
			assert!(matches!(
				context.nth_notification(3),
				SubscriberNotification::Unsubscribe
			));
			assert!(!context.nth_notification_exists(4));
			assert_eq!(context.all_observed_values(), vec![1, 2]);

			assert_eq!(tracked_data.read_next_count(0), 3); // There's one extra due to a peek, but it's clearly less than 3
			assert!(!tracked_data.is_finished(0));

			subscription.unsubscribe(&mut context);
		}
	}
}
