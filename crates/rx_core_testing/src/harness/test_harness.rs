use std::{
	fmt::Debug,
	marker::PhantomData,
	sync::{Arc, Mutex, MutexGuard},
};

use derive_where::derive_where;
use rx_core_common::{
	LockWithPoisonBehavior, Observable, RxObserver, Signal, Subscriber, SubscriberNotification,
	SubscriptionLike, SubscriptionWithTeardown, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{
	MockObserver, NotificationCollector, NotificationCollectorState, TeardownTracker, TestError,
	TestSubject, TrackedTeardownSubscriptionExtension,
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
	Source: Observable,
	FinalOut: Signal + Clone,
	FinalOutError: Signal + Clone,
{
	prefix: &'static str,
	subject_source: TestSubject<Source::Out, Source::OutError>,
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
	TestHarness<TestSubject<In, InError>, FinalOut, FinalOutError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	FinalOut: Signal + Clone + PartialEq + Debug,
	FinalOutError: Signal + Clone + PartialEq + Debug,
{
	pub fn new(prefix: &'static str) -> Self {
		let subject_source = TestSubject::<In, InError>::default();
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

	pub fn source(&mut self) -> &mut TestSubject<In, InError> {
		&mut self.subject_source
	}
}

impl<Source, FinalOut, FinalOutError> TestHarness<Source, FinalOut, FinalOutError>
where
	Source: Observable,
	FinalOut: Signal + Clone + PartialEq + Debug,
	FinalOutError: Signal + Clone + PartialEq + Debug,
{
	pub fn new_with_source(prefix: &'static str, source: Source) -> Self {
		Self {
			prefix,
			subject_source: TestSubject::default(),
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

	pub fn create_harness_destination(
		&mut self,
		take_count: Option<usize>,
	) -> HarnessDestination<FinalOut, FinalOutError> {
		let (destination, tracked_teardown_downstream) =
			HarnessDestination::new(self.notification_collector.clone(), self.prefix, take_count);
		self.tracked_teardown_downstream = Some(tracked_teardown_downstream);
		destination
	}

	pub fn subscribe_to(
		&mut self,
		mut observable: impl Observable<Out = FinalOut, OutError = FinalOutError>,
	) {
		let destination = self.create_harness_destination(None);
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

	/// Verifies:
	/// - rx_verify_upstream_teardowns_executed
	/// - rx_verify_downstream_teardowns_executed
	/// - rx_verify_subscription_teardowns_executed
	#[track_caller]
	pub fn assert_terminal_notification(
		&mut self,
		terminal_notification: SubscriberNotification<FinalOut, FinalOutError>,
	) {
		let last_notification_index =
			self.notifications()
				.len()
				.checked_sub(1)
				.unwrap_or_else(|| {
					panic!(
						"{} - rx_verify_closed - at least one notification should've happen!",
						self.prefix
					)
				});

		let verification_code = match terminal_notification {
			SubscriberNotification::Complete => "rx_verify_completed",
			SubscriberNotification::Error(_) => "rx_verify_errored",
			SubscriberNotification::Unsubscribe => "rx_verify_unsubscribed",
			SubscriberNotification::Next(_) => panic!("Next is not a terminal notification!"),
		};

		self.notifications().assert_notifications(
			&format!(
				"{} - {} - Did not observe expected last notification!",
				self.prefix, verification_code
			),
			last_notification_index,
			[terminal_notification],
			true,
		);

		assert!(
			self.get_subscription().is_closed(),
			"{} - rx_verify_closed - Subscription did not close after last notification!",
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
	Source: Observable,
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
	Source: Observable,
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
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct HarnessDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[destination]
	destination: MockObserver<In, InError>,
	take_count: Option<usize>,
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
		take_count: Option<usize>,
	) -> (Self, TeardownTracker) {
		let mut destination = MockObserver::new(notification_collector);
		let tracked_teardown_downstream = destination.add_tracked_teardown(&format!(
			"{prefix} - rx_verify_downstream_teardowns_executed"
		));

		(
			Self {
				destination,
				take_count,
				_phantom_data: PhantomData,
			},
			tracked_teardown_downstream,
		)
	}
}

impl<In, InError> RxObserver for HarnessDestination<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if let Some(count) = self.take_count.as_mut() {
			if *count > 0 {
				*count -= 1;
				self.destination.next(next);

				if *count == 0 && !self.is_closed() {
					self.complete();
				}
			}
		} else {
			self.destination.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
