use std::{
	fmt::Debug,
	marker::PhantomData,
	sync::{Arc, Mutex, MutexGuard},
};

use derive_where::derive_where;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, Observer, Signal, Subscriber, SubscriberNotification,
	SubscriptionLike, SubscriptionWithTeardown, UpgradeableObserver,
};

use crate::{
	MockObserver, NotificationCollector, NotificationCollectorState, SingleSubscriberSubject,
	TeardownTracker, TestError, TrackedTeardownSubscriptionExtension,
};

const TEST_HARNESS_MISSING_TRACKER_UPSTREAM: &str =
	"upstream teardown tracker does not exist, forgot to";
const TEST_HARNESS_MISSING_TRACKER_DOWNSTREAM: &str =
	"downstream teardown tracker does not exist, forgot to";
const TEST_HARNESS_MISSING_TRACKER_SUBSCRIPTION: &str =
	"subscription teardown tracker does not exist, forgot to";

#[derive_where(Debug; Source::Out, Source::OutError, FinalOut, FinalOutError)]
pub struct TestHarness<Source, FinalOut = usize, FinalOutError = TestError>
where
	Source: 'static + Observable + Send + Sync,
	FinalOut: Signal + Clone,
	FinalOutError: Signal + Clone,
{
	prefix: &'static str,
	subject_source: SingleSubscriberSubject<Source::Out, Source::OutError>,
	#[derive_where(skip(Debug))]
	source: Option<Source>,
	notification_collector: NotificationCollector<FinalOut, FinalOutError>,
	tracked_teardown_upstream: Arc<Mutex<Option<TeardownTracker>>>,
	tracked_teardown_downstream: Option<TeardownTracker>,
	tracked_teardown_subscription: Option<TeardownTracker>,
	#[derive_where(skip(Debug))]
	subscription: Option<Box<dyn SubscriptionWithTeardown + Send + Sync>>,
	_phantom_data: PhantomData<fn(FinalOut, FinalOutError) -> (FinalOut, FinalOutError)>,
}

impl<In, InError, FinalOut, FinalOutError>
	TestHarness<SingleSubscriberSubject<In, InError>, FinalOut, FinalOutError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	FinalOut: Signal + Clone + PartialEq + Debug,
	FinalOutError: Signal + Clone + PartialEq + Debug,
{
	pub fn new_operator_harness(prefix: &'static str) -> Self {
		let subject_source = SingleSubscriberSubject::<In, InError>::default();
		Self {
			prefix,
			source: Some(subject_source.clone()),
			notification_collector: NotificationCollector::default(),
			subject_source,
			tracked_teardown_upstream: Arc::new(Mutex::new(None)),
			tracked_teardown_downstream: None,
			tracked_teardown_subscription: None,
			subscription: None,
			_phantom_data: PhantomData,
		}
	}

	pub fn source(&mut self) -> &mut SingleSubscriberSubject<In, InError> {
		&mut self.subject_source
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_error(
		&mut self,
		in_error: InError,
		out_error: FinalOutError,
	) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before error!",
			self.prefix
		);

		self.subject_source.error(in_error);

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

		self.tracked_teardown_upstream
			.lock_ignore_poison()
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_UPSTREAM)
			.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_DOWNSTREAM)
			.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_SUBSCRIPTION)
			.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_complete(&mut self) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before complete!",
			self.prefix
		);

		self.subject_source.complete();

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

		self.tracked_teardown_upstream
			.lock_ignore_poison()
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_UPSTREAM)
			.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_DOWNSTREAM)
			.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_SUBSCRIPTION)
			.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}

	#[track_caller]
	pub fn assert_rx_contract_closed_after_unsubscribe(&mut self) {
		let notification_index_so_far = self.notifications().len();

		assert!(
			!self.get_subscription().is_closed(),
			"{} - should not have been closed before unsubscribe!",
			self.prefix
		);

		self.subject_source.unsubscribe();

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

		self.tracked_teardown_upstream
			.lock_ignore_poison()
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_UPSTREAM)
			.assert_was_torn_down(); // rx_verify_upstream_teardowns_executed
		self.tracked_teardown_downstream
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_DOWNSTREAM)
			.assert_was_torn_down(); // rx_verify_downstream_teardowns_executed
		self.tracked_teardown_subscription
			.as_ref()
			.expect(TEST_HARNESS_MISSING_TRACKER_SUBSCRIPTION)
			.assert_was_torn_down(); // rx_verify_subscription_teardowns_executed
	}
}

impl<Source, FinalOut, FinalOutError> TestHarness<Source, FinalOut, FinalOutError>
where
	Source: 'static + Observable + Send + Sync,
	FinalOut: Signal + Clone,
	FinalOutError: Signal + Clone,
{
	pub fn new_observable_harness(prefix: &'static str, source: Source) -> Self {
		Self {
			prefix,
			subject_source: SingleSubscriberSubject::default(),
			notification_collector: NotificationCollector::default(),
			source: Some(source),
			tracked_teardown_upstream: Arc::new(Mutex::new(None)),
			tracked_teardown_downstream: None,
			tracked_teardown_subscription: None,
			subscription: None,
			_phantom_data: PhantomData,
		}
	}

	pub fn create_harness_observable(
		&mut self,
	) -> HarnessObservable<Source, FinalOut, FinalOutError> {
		let source = self.source.take().expect("Source is already taken!");
		HarnessObservable::new(self.prefix, source, self.tracked_teardown_upstream.clone())
	}

	pub fn create_harness_destination(&mut self) -> HarnessDestination<FinalOut, FinalOutError> {
		let (destination, tracked_teardown_downstream) =
			HarnessDestination::new(self.notification_collector.clone(), self.prefix);
		self.tracked_teardown_downstream = Some(tracked_teardown_downstream);
		destination
	}

	pub fn subscribe_to(
		&mut self,
		mut observable: impl Observable<Out = FinalOut, OutError = FinalOutError>,
	) {
		let destination = self.create_harness_destination();
		self.register_subscription(observable.subscribe(destination));
	}

	pub fn register_subscription(
		&mut self,
		mut subscription: impl 'static + SubscriptionWithTeardown + Send + Sync,
	) {
		if self.tracked_teardown_subscription.is_some() || self.subscription.is_some() {
			panic!("A subscription was already registered!");
		}

		self.tracked_teardown_subscription = Some(subscription.add_tracked_teardown(&format!(
			"{prefix} - rx_verify_subscription_teardowns_executed",
			prefix = self.prefix
		)));

		self.subscription = Some(Box::new(subscription))
	}

	pub fn notifications(
		&self,
	) -> MutexGuard<'_, NotificationCollectorState<FinalOut, FinalOutError>> {
		self.notification_collector.lock()
	}

	pub fn get_subscription(&self) -> &(dyn SubscriptionWithTeardown + Send + Sync) {
		self.subscription
			.as_ref()
			.unwrap_or_else(|| panic!("{} - subscription should exist", self.prefix))
			.as_ref()
	}

	pub fn get_subscription_mut(&mut self) -> &mut (dyn SubscriptionWithTeardown + Send + Sync) {
		self.subscription
			.as_mut()
			.unwrap_or_else(|| panic!("{} - subscription should exist", self.prefix))
			.as_mut()
	}

	pub fn is_subscription_closed(&self) -> bool {
		self.get_subscription().is_closed()
	}
}

#[derive(RxObservable)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
pub struct HarnessObservable<Source, FinalOut, FinalOutError>
where
	Source: Observable,
{
	prefix: &'static str,
	source: Source,
	tracked_teardown_upstream: Arc<Mutex<Option<TeardownTracker>>>,
	_phantom_data: PhantomData<fn(FinalOut, FinalOutError) -> (FinalOut, FinalOutError)>,
}

impl<Source, FinalOut, FinalOutError> HarnessObservable<Source, FinalOut, FinalOutError>
where
	Source: 'static + Observable + Send + Sync,
	FinalOut: Signal,
	FinalOutError: Signal,
{
	pub fn new(
		prefix: &'static str,
		source: Source,
		tracked_teardown_upstream: Arc<Mutex<Option<TeardownTracker>>>,
	) -> Self {
		Self {
			prefix,
			source,
			tracked_teardown_upstream,
			_phantom_data: PhantomData,
		}
	}
}

impl<Source, FinalOut, FinalOutError> Observable
	for HarnessObservable<Source, FinalOut, FinalOutError>
where
	Source: 'static + Observable<Out = FinalOut, OutError = FinalOutError> + Send + Sync,
	FinalOut: Signal,
	FinalOutError: Signal,
{
	type Subscription<Destination>
		= Source::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut subscription = self.source.subscribe(destination);
		self.tracked_teardown_upstream.lock_ignore_poison().replace(
			subscription.add_tracked_teardown(&format!(
				"{prefix} - rx_verify_upstream_teardowns_executed",
				prefix = self.prefix
			)),
		);
		subscription
	}
}

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_upgrades_to(self)]
#[rx_delegate_observer_to_destination]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct HarnessDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[destination]
	mock_observer: MockObserver<In, InError>,
	_phantom_data: PhantomData<fn(In, InError) -> (In, InError)>,
}

impl<In, InError> HarnessDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn new(
		notification_collector: NotificationCollector<In, InError>,
		prefix: &str,
	) -> (Self, TeardownTracker) {
		let mut mock_observer = MockObserver::new(notification_collector);
		let tracked_teardown_downstream = mock_observer.add_tracked_teardown(&format!(
			"{prefix} - rx_verify_downstream_teardowns_executed"
		));

		(
			Self {
				mock_observer,
				_phantom_data: PhantomData,
			},
			tracked_teardown_downstream,
		)
	}
}
