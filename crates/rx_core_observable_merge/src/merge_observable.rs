use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_operator_map_into::MapIntoSubscriber;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Signal, Subscriber, SubscriptionData, TeardownCollection, UpgradeableObserver,
};

#[derive(RxObservable, Clone, Debug)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct MergeObservable<Out, OutError, O1, O2>
where
	Out: Signal,
	OutError: Signal,
	O1: 'static + Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: 'static + Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	observable_1: O1,
	observable_2: O2,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, O1, O2> MergeObservable<Out, OutError, O1, O2>
where
	Out: Signal,
	OutError: Signal,
	O1: 'static + Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: 'static + Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	pub fn new(observable_1: O1, observable_2: O2) -> Self {
		Self {
			observable_1,
			observable_2,
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, O1, O2> Observable for MergeObservable<Out, OutError, O1, O2>
where
	Out: Signal,
	OutError: Signal,
	O1: 'static + Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: 'static + Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
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
		let rc_subscriber = RcSubscriber::new(destination);

		let s1 = self
			.observable_1
			.subscribe(MapIntoSubscriber::new(rc_subscriber.clone()));

		let s2 = self
			.observable_2
			.subscribe(MapIntoSubscriber::new(rc_subscriber));

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(s1.into());
		subscription.add_teardown(s2.into());
		subscription
	}
}
