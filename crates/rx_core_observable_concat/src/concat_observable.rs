use core::{marker::PhantomData, num::NonZero};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_observable_erased::{ErasedObservables, observable::ErasedObservable};
use rx_core_subscriber_concurrent::ConcurrentSubscriberProvider;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionData, TeardownCollection,
	UpgradeableObserver,
};

#[derive(RxObservable, Clone)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct ConcatObservable<Out, OutError, const SIZE: usize>
where
	Out: Signal,
	OutError: Signal,
{
	observables: ErasedObservables<Out, OutError, SIZE>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, const SIZE: usize> ConcatObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	pub fn new(observables: impl Into<ErasedObservables<Out, OutError, SIZE>>) -> Self {
		Self {
			observables: observables.into(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, const SIZE: usize> Observable for ConcatObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	type Subscription<Destination>
		= SubscriptionData
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let destination = observer.upgrade();

		let mut concat_subscriber = HigherOrderAllSubscriber::<
			ErasedObservable<Out, OutError>,
			OutError,
			ConcurrentSubscriberProvider,
			<Destination as UpgradeableObserver>::Upgraded,
		>::new(destination, NonZero::<usize>::MIN);

		for next_observable in self.observables.iter().cloned() {
			concat_subscriber.next(next_observable);
		}
		concat_subscriber.complete();

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(concat_subscriber.into());
		subscription
	}
}

#[cfg(test)]
mod test {

	use rx_core::prelude::*;
	use rx_core_testing::prelude::*;
	use rx_core_traits::{Observable, SubscriberNotification};

	use crate::observable::ConcatObservable;

	#[test]
	fn should_complete_if_all_inputs_complete() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<usize>::default();
		let mut subject_3 = PublishSubject::<usize>::default();

		let mut subscription =
			ConcatObservable::new((subject_1.clone(), subject_2.clone(), subject_3.clone()))
				.subscribe(destination);

		assert!(
			notification_collector.lock().is_empty(),
			"nothing should happen when subscribed to non replaying sources"
		);

		subject_1.next(1);

		assert_eq!(
			notification_collector.lock().nth_notification(0),
			&SubscriberNotification::Next(1)
		);

		subject_2.next(2);

		assert!(
			!notification_collector.lock().nth_notification_exists(1),
			"should not be subscribed to the second source until the first one completes"
		);

		subject_1.complete();
		subject_3.complete(); // This will never emit

		subject_2.next(2);

		assert_eq!(
			notification_collector.lock().nth_notification(1),
			&SubscriberNotification::Next(2)
		);

		subject_2.next(3);

		assert_eq!(
			notification_collector.lock().nth_notification(2),
			&SubscriberNotification::Next(3)
		);

		subject_2.complete();

		assert_eq!(
			notification_collector.lock().nth_notification(3),
			&SubscriberNotification::Complete,
			"downstream should complete when all upstream observables complete"
		);

		subscription.unsubscribe();
	}

	#[test]
	fn should_immediately_complete_all_inputs_immediately_complete() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		subject_1.complete();
		let mut subject_2 = PublishSubject::<usize>::default();
		subject_2.complete();

		let mut subscription =
			ConcatObservable::new((subject_1.clone(), subject_2.clone())).subscribe(destination);

		assert_eq!(
			notification_collector.lock().nth_notification(0),
			&SubscriberNotification::Complete,
			"downstream should complete when all upstream observables complete"
		);

		subscription.unsubscribe();
		assert_eq!(
			notification_collector.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"downstream should unsubscribe when the subscription unsubscribes"
		);
	}
}
