use rx_core_emission_variants::{
	EitherOutError2, IntoVariant1of2Subscriber, IntoVariant2of2Subscriber,
};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Subscriber, SubscriptionData, TeardownCollection, UpgradeableObserver,
};

use crate::CombineLatestSubscriber;

#[derive(RxObservable)]
#[rx_out((O1::Out, O2::Out))]
#[rx_out_error( EitherOutError2<O1, O2>)]
pub struct CombineLatestObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> CombineLatestObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(observable_1: O1, observable_2: O2) -> Self {
		Self {
			observable_1,
			observable_2,
		}
	}
}

impl<O1, O2> Observable for CombineLatestObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
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
		let rc_subscriber =
			RcSubscriber::new(CombineLatestSubscriber::<_, O1, O2>::new(destination));

		let s1 = self
			.observable_1
			.subscribe(IntoVariant1of2Subscriber::new(rc_subscriber.clone()));

		let s2 = self
			.observable_2
			.subscribe(IntoVariant2of2Subscriber::new(rc_subscriber.clone()));

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(s1.into());
		subscription.add_teardown(s2.into());
		subscription
	}
}
