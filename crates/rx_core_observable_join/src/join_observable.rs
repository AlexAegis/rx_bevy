use rx_core_macro_observable_derive::RxObservable;
use rx_core_notification_variadics::{
	EitherNotificationSelector1Of2, EitherNotificationSelector2Of2, EitherSubscriber2,
};
use rx_core_traits::{
	Observable, SharedSubscriber, Subscriber, SubscriptionData, TeardownCollection,
	UpgradeableObserver,
};

use crate::JoinSubscriber;

/// # [JoinObservable]
///
/// This observable will only emit once both of it's input observables
/// have completed. After which it will emit a tuple of the last emissions
/// from each input observable, then complete.
///
/// Meaning if even one of the observables haven't emitted before all of them
/// had completed, only a complete notification will be observed!
///
/// If not all observables complete, nothing will be emitted even if all
/// input observables were primed.
#[derive(RxObservable)]
#[rx_out((O1::Out, O2::Out))]
#[rx_out_error(O1::OutError)]
pub struct JoinObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> JoinObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	pub fn new(o1: O1, o2: O2) -> Self {
		Self {
			observable_1: o1,
			observable_2: o2,
		}
	}
}

impl<O1, O2> Observable for JoinObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
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
		let shared_subscriber =
			SharedSubscriber::new(JoinSubscriber::<_, O1, O2>::new(destination));

		let s1 = self.observable_1.subscribe(EitherSubscriber2::<
			EitherNotificationSelector1Of2<O1, O2>,
			_,
			O1,
			O2,
		>::new(shared_subscriber.clone()));

		let s2 = self.observable_2.subscribe(EitherSubscriber2::<
			EitherNotificationSelector2Of2<O1, O2>,
			_,
			O1,
			O2,
		>::new(shared_subscriber.clone()));

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(s1.into());
		subscription.add_teardown(s2.into());
		subscription
	}
}
