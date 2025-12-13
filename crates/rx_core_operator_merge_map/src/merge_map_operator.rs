use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_map::HigherOrderMapSubscriber;
use rx_core_subscriber_merge::MergeSubscriberProvider;
use rx_core_traits::{Observable, Operator, Signal, Subscriber};

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct MergeMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Mapper, InnerObservable> MergeMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, InnerObservable> Operator
	for MergeMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		MergeSubscriberProvider,
		Destination,
	>
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
		HigherOrderMapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, InnerObservable> Clone
	for MergeMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
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

		let mut subject = Subject::<i32, Never>::default();
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
		assert!(matches!(
			notification_collector.lock().nth_notification(7),
			&SubscriberNotification::Unsubscribe
		));

		subscription.unsubscribe();
		subject.unsubscribe();
	}

	#[test]
	fn upstream_ticks_are_forwarded_to_the_inner_subscription() {
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut subject = Subject::<i32, Never>::default();
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
