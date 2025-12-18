use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_concurrent::ConcurrentSubscriberProvider;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In::Out)]
#[rx_out_error(In::OutError)]
pub struct MergeAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	concurrency_limit: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> MergeAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	pub fn new(concurrency_limit: usize) -> Self {
		Self {
			concurrency_limit,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for MergeAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	type Subscriber<Destination>
		= HigherOrderAllSubscriber<In, InError, ConcurrentSubscriberProvider, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		HigherOrderAllSubscriber::new(destination, self.concurrency_limit)
	}
}

#[cfg(test)]
mod test {

	use rx_core::prelude::*;
	use rx_core_testing::prelude::*;
	use rx_core_traits::SubscriberNotification;

	#[test]
	fn subscribes_to_the_inner_observable_as_many_times_as_many_upstream_emissions_there_are() {
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut source = (1..=2)
			.into_observable()
			.switch_map(|_| (10..=12).into_observable());
		let mut subscription = source.subscribe(mock_destination);
		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(
			notification_collector.lock().all_observed_values(),
			vec![10, 11, 12, 10, 11, 12]
		);
		subscription.unsubscribe();
	}

	#[test]
	fn subscribes_to_the_inner_observable_on_every_emit_of_a_source_subject_and_completes() {
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut subject = PublishSubject::<i32, Never>::default();
		let mut source = subject.clone().switch_map(|i| (0..=i).into_observable());
		let mut subscription = source.subscribe(mock_destination);

		subject.next(1);

		assert_eq!(
			notification_collector.lock().all_observed_values(),
			vec![0, 1]
		);

		subject.next(3);
		assert_eq!(
			notification_collector.lock().all_observed_values(),
			vec![0, 1, 0, 1, 2, 3]
		);

		subject.complete();

		assert!(matches!(
			notification_collector.lock().nth_notification(6),
			&SubscriberNotification::Complete
		));

		subscription.unsubscribe();

		assert!(matches!(
			notification_collector.lock().nth_notification(7),
			&SubscriberNotification::Unsubscribe
		));

		subject.unsubscribe();
	}

	#[test]
	fn upstream_ticks_are_forwarded_to_the_inner_subscription() {
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut subject = PublishSubject::<i32, Never>::default();
		let mut source = subject.clone().switch_map(|i| (0..=i).into_observable());
		let mut subscription = source.subscribe(mock_destination);

		subject.next(1);

		assert_eq!(
			notification_collector.lock().all_observed_values(),
			vec![0, 1]
		);

		subject.next(3);

		assert_eq!(
			notification_collector.lock().all_observed_values(),
			vec![0, 1, 0, 1, 2, 3]
		);

		subject.unsubscribe();
		subscription.unsubscribe();
	}
}
