use core::{marker::PhantomData, num::NonZero};

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_map::HigherOrderMapSubscriber;
use rx_core_subscriber_switch::SwitchSubscriberProvider;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

/// # [switch_map][SwitchMapOperator]
///
/// > Category: Higher Order Operator
///
/// The `switch_map` subscribes to incoming observables immediately,
/// unsubscribing the existing inner subscription if there were any.
///
/// - The `switch_map` can only have at most one active inner subscriptions.
/// - The `switch_map` is a `map` and a `switch_all` operator combined where
///   `map` returns an observable.
///
/// ## Higher Order Operators
///
/// Higher Order Operators are operators that operator over a stream of
/// observables. All they do is subscribe to incoming observables, and what
/// they differ in is what happens with the inner observable and the incoming
/// next inner observable when one is received.
///
/// The higher order operators are:
/// - [concat_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_all)
/// - [concat_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_map)
/// - [exhaust_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_all)
/// - [exhaust_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_map)
/// - [merge_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_all)
/// - [merge_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_map)
/// - [switch_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_all)
/// - [switch_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_map)
///
/// ### Mandatory Error Mapper for all Higher Order Operators
///
/// Since the inner observables', and the upstream error type can differ, and
/// errors have to be able to go forward unless explicitly caught, a mapping
/// between the two types must be defined. Ideally, this would be a simple
/// `.into()` transformation, but the current `Never` type, `Infallible`
/// does not implement `impl<T> From<Infallible> for T`, which could always
/// be an `unreachable!()` for any `T`. But this is intentionally reserved for
/// the actual never type `!` once it stabilizes.
///
/// Currently the error mapper in higher order operators is a necessary
/// evil. Without it, a never erroring source of erroring observables is
/// impossible to use with higher order operators.
///
/// ```text
/// // Where subject_1 and subject_2 has an error type of `MyError`
/// [subject_1, subject_2].into_observables().concat_all(); // Impossible: Infallible is not Into<MyError>
/// ```
///
/// #### Future Migration
///
/// In a future major release, once `!` stabilizes, the ErrorMapper will be
/// removed, in favor of using `.into()` internally. Wherever actual error
/// mapping is required, an additional `map_error` operator can be used. Uses
/// of `Never::error_mapper()` (or manual definitions of `|_| unreachable!()`)
/// can simply be removed.
#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct SwitchMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + Fn(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	mapper: Mapper,
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Mapper, ErrorMapper, InnerObservable>
	SwitchMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + Fn(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(mapper: Mapper, error_mapper: ErrorMapper) -> Self {
		Self {
			mapper,
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, ErrorMapper, InnerObservable> ComposableOperator
	for SwitchMapOperator<In, InError, Mapper, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	ErrorMapper: 'static + Fn(InError) -> InnerObservable::OutError + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		SwitchSubscriberProvider,
		ErrorMapper,
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
		HigherOrderMapSubscriber::new(
			destination,
			self.mapper.clone(),
			self.error_mapper.clone(),
			NonZero::<usize>::MIN,
		)
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
			.switch_map(|_| (10..=12).into_observable(), |_| unreachable!());
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
		let mut source = subject
			.clone()
			.switch_map(|i| (0..=i).into_observable(), |_| unreachable!());
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
		let mut source = subject
			.clone()
			.switch_map(|i| (0..=i).into_observable(), |_| unreachable!());
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
