use std::{fmt::Debug, marker::PhantomData, sync::MutexGuard};

use rx_core_traits::{
	Observable, ObservableOutput, Observer, Operator, Pipe, SharedSubscription, Signal,
	SubscriberNotification, SubscriptionLike,
};

use crate::{
	ErasedFinalizeOperator, MockObserver, NotificationCollector, NotificationCollectorState,
	SingleSubscriberSubject, TeardownTracker, TrackedTeardownSubscriptionExtension,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestError;

pub struct OperatorTestHarness<OutputObservable, In = usize, InError = TestError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	OutputObservable: Observable,
{
	source: SingleSubscriberSubject<In, InError>,
	prefix: &'static str,
	notification_collector:
		NotificationCollector<OutputObservable::Out, OutputObservable::OutError>,
	tracked_teardown_upstream: TeardownTracker,
	tracked_teardown_downstream: TeardownTracker,
	tracked_teardown_subscription: TeardownTracker,
	subscription: Option<
		<OutputObservable as Observable>::Subscription<
			MockObserver<
				<OutputObservable as ObservableOutput>::Out,
				<OutputObservable as ObservableOutput>::OutError,
			>,
		>,
	>,
	_phantom_data: PhantomData<OutputObservable>,
}

impl<OutputObservable, In, InError> OperatorTestHarness<OutputObservable, In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	OutputObservable: Observable + Send + Sync,
	OutputObservable::Out: Debug + PartialEq,
	OutputObservable::OutError: Debug + PartialEq,
{
	pub fn new(
		prefix: &'static str,
		apply: impl FnOnce(
			Pipe<SingleSubscriberSubject<In, InError>, ErasedFinalizeOperator<In, InError>>,
		) -> OutputObservable,
	) -> Self {
		// TODO: Make all this buildable
		let destination =
			MockObserver::<OutputObservable::Out, OutputObservable::OutError>::default();
		let notification_collector = destination.get_notification_collector();

		let source = SingleSubscriberSubject::<In, InError>::default();

		let mut tracker_subscription_upstream = SharedSubscription::default();
		let mut tracker_subscription_downstream = SharedSubscription::default();
		let tracked_teardown_upstream = tracker_subscription_upstream
			.add_tracked_teardown(&format!("{prefix} - rx_verify_upstream_teardowns_executed"));
		let tracked_teardown_downstream = tracker_subscription_downstream.add_tracked_teardown(
			&format!("{prefix} - rx_verify_downstream_teardowns_executed"),
		);

		let output_observable = apply(
			ErasedFinalizeOperator::new(move || tracker_subscription_upstream.unsubscribe())
				.operate(source.clone()),
		);

		let mut with_operator =
			ErasedFinalizeOperator::new(move || tracker_subscription_downstream.unsubscribe())
				.operate(output_observable);

		let mut subscription = with_operator.subscribe(destination);

		let tracked_teardown_subscription = subscription.add_tracked_teardown(&format!(
			"{prefix} - rx_verify_subscription_teardowns_executed"
		));

		Self {
			source,
			prefix,
			notification_collector,
			tracked_teardown_upstream,
			tracked_teardown_downstream,
			tracked_teardown_subscription,
			subscription: Some(subscription),
			_phantom_data: PhantomData,
		}
	}

	pub fn source(&mut self) -> &mut SingleSubscriberSubject<In, InError> {
		&mut self.source
	}

	pub fn notifications(
		&self,
	) -> MutexGuard<'_, NotificationCollectorState<OutputObservable::Out, OutputObservable::OutError>>
	{
		self.notification_collector.lock()
	}

	pub fn get_subscription(
		&self,
	) -> &<OutputObservable as Observable>::Subscription<
		MockObserver<
			<OutputObservable as ObservableOutput>::Out,
			<OutputObservable as ObservableOutput>::OutError,
		>,
	> {
		self.subscription
			.as_ref()
			.unwrap_or_else(|| panic!("{} - subscription should exist", self.prefix))
	}

	pub fn get_subscription_mut(
		&mut self,
	) -> &mut <OutputObservable as Observable>::Subscription<
		MockObserver<
			<OutputObservable as ObservableOutput>::Out,
			<OutputObservable as ObservableOutput>::OutError,
		>,
	> {
		self.subscription
			.as_mut()
			.unwrap_or_else(|| panic!("{} - subscription should exist", self.prefix))
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_error(
		&mut self,
		in_error: InError,
		out_error: OutputObservable::OutError,
	) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before error!",
			self.prefix
		);

		self.source.error(in_error);

		self.notifications().assert_notifications(
			&format!(
				"{} - rx_verify_errored - Did not observe an error notification!",
				self.prefix
			),
			notification_index_so_far,
			[SubscriberNotification::Error(out_error)],
			true,
		);

		assert!(
			self.get_subscription().is_closed(),
			"{} - rx_verify_closed - Subscription did not close after error!",
			self.prefix
		);

		self.tracked_teardown_upstream.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_complete(&mut self) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before complete!",
			self.prefix
		);

		self.source.complete();

		self.notifications().assert_notifications(
			&format!(
				"{} - rx_verify_completed - Did not observe a complete notification!",
				self.prefix
			),
			notification_index_so_far,
			[SubscriberNotification::Complete],
			true,
		);

		assert!(
			self.get_subscription().is_closed(),
			"{} - rx_verify_closed - Subscription did not close after complete!",
			self.prefix
		);

		self.tracked_teardown_upstream.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_unsubscribe(&mut self) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before unsubscribe!",
			self.prefix
		);

		self.source.unsubscribe();

		self.notifications().assert_notifications(
			&format!(
				"{} - rx_verify_unsubscribed - Did not observe an unsubscribe notification!",
				self.prefix
			),
			notification_index_so_far,
			[SubscriberNotification::Unsubscribe],
			true,
		);

		assert!(
			self.get_subscription().is_closed(),
			"{} - rx_verify_closed - Subscription did not close after unsubscribe!",
			self.prefix
		);

		self.tracked_teardown_upstream.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}
}
