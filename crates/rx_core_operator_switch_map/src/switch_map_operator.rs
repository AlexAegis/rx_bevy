use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Observable, Operator, Signal, Subscriber, SubscriptionContext};

use crate::SwitchMapSubscriber;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
#[rx_context(InnerObservable::Context)]
pub struct SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Switcher, InnerObservable>
	SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable> Operator
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		SwitchMapSubscriber::new(destination, self.switcher.clone(), context)
	}
}

impl<In, InError, Switcher, InnerObservable> Clone
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
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
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32>::default();

		let mut source = (1..=2)
			.into_observable::<MockContext<_, _, _>>()
			.switch_map(|_| (10..=12).into_observable::<MockContext<_, _, _>>());
		let mut subscription = source.subscribe(mock_destination, &mut context);
		assert!(
			context.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(context.all_observed_values(), vec![10, 11, 12, 10, 11, 12]);
		subscription.unsubscribe(&mut context);
	}

	#[test]
	fn subscribes_to_the_inner_observable_on_every_emit_of_a_source_subject_and_completes() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32>::default();

		let mut subject = Subject::<i32, Never, MockContext<i32>>::default();
		let mut source = subject
			.clone()
			.switch_map(|i| (0..=i).into_observable::<MockContext<_, _, _>>());
		let mut subscription = source.subscribe(mock_destination, &mut context);

		subject.next(1, &mut context);

		assert_eq!(context.all_observed_values(), vec![0, 1]);

		subject.next(3, &mut context);
		assert_eq!(context.all_observed_values(), vec![0, 1, 0, 1, 2, 3]);

		subject.complete(&mut context);

		assert!(matches!(
			context.nth_notification(6),
			&SubscriberNotification::Complete
		));
		assert!(matches!(
			context.nth_notification(7),
			&SubscriberNotification::Unsubscribe
		));

		subscription.unsubscribe(&mut context);
		subject.unsubscribe(&mut context);
	}

	#[test]
	fn upstream_ticks_are_forwarded_to_the_inner_subscription() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32>::default();

		let mut subject = Subject::<i32, Never, MockContext<i32>>::default();
		let mut source = subject
			.clone()
			.switch_map(|i| (0..=i).into_observable::<MockContext<_, _, _>>());
		let mut subscription = source.subscribe(mock_destination, &mut context);

		subject.next(1, &mut context);
		println!("{:?}", context);
		assert_eq!(context.all_observed_values(), vec![0, 1]);

		subject.next(3, &mut context);
		println!("{:?}", context);
		assert_eq!(context.all_observed_values(), vec![0, 1, 0, 1, 2, 3]);

		subject.unsubscribe(&mut context);
		subscription.unsubscribe(&mut context);
	}
}
