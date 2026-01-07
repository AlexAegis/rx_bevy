use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_common::{
	ErasedSubscriber, LockWithPoisonBehavior, Observable, Observer, SharedSubscriber, Signal,
	Subscriber, SubscriptionLike, UpgradeableObserver,
};
use rx_core_macro_subject_derive::RxSubject;
use rx_core_macro_subscription_derive::RxSubscription;

const EXPECT_ACTIVE_SUBSCRIPTION: &str = "Subscription to be active!";

/// # [TestSubject]
///
/// It does not do multicasting, but does let you interact with the latest
/// subscriber like a subject would.
///
/// Will panic if there is no active subscription!
///
/// Used and made for testing only!
#[derive_where(Default, Clone)]
#[derive(RxSubject, Debug)]
#[rx_in(Out)]
#[rx_in_error(OutError)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct TestSubject<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	subscriber: Arc<Mutex<Option<SharedSubscriber<ErasedSubscriber<Out, OutError>>>>>,
}

impl<Out, OutError> Observer for TestSubject<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.subscriber
			.lock_ignore_poison()
			.as_mut()
			.expect(EXPECT_ACTIVE_SUBSCRIPTION)
			.next(next);
	}

	#[inline]
	#[track_caller]
	fn error(&mut self, error: Self::InError) {
		self.subscriber
			.lock_ignore_poison()
			.as_mut()
			.expect(EXPECT_ACTIVE_SUBSCRIPTION)
			.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subscriber
			.lock_ignore_poison()
			.as_mut()
			.expect(EXPECT_ACTIVE_SUBSCRIPTION)
			.complete();
	}
}

impl<Out, OutError> SubscriptionLike for TestSubject<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscriber
			.lock_ignore_poison()
			.as_ref()
			.expect(EXPECT_ACTIVE_SUBSCRIPTION)
			.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.subscriber
			.lock_ignore_poison()
			.as_mut()
			.expect(EXPECT_ACTIVE_SUBSCRIPTION)
			.unsubscribe();
	}
}

impl<Out, OutError> Observable for TestSubject<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	type Subscription<Destination>
		= NotifiableSubscription<Out, OutError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let shared_subscriber = SharedSubscriber::new(ErasedSubscriber::new(destination.upgrade()));
		self.subscriber
			.lock_ignore_poison()
			.replace(shared_subscriber.clone());

		NotifiableSubscription::new(shared_subscriber)
	}
}

#[derive(RxSubscription)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct NotifiableSubscription<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[destination]
	destination: SharedSubscriber<ErasedSubscriber<Out, OutError>>,
}

impl<Out, OutError> NotifiableSubscription<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	pub fn new(destination: SharedSubscriber<ErasedSubscriber<Out, OutError>>) -> Self {
		Self { destination }
	}
}
