use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, RcSubscriber, Subscription, Teardown, UpgradeableObserver,
};
use rx_bevy_operator_map_into::MapIntoSubscriber;

pub fn merge<Out, OutError, O1, O2>(
	observable_1: O1,
	observable_2: O2,
) -> MergeObservable<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	MergeObservable::new(observable_1, observable_2)
}

pub struct MergeObservable<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	observable_1: O1,
	observable_2: O2,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, O1, O2> MergeObservable<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
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

impl<Out, OutError, O1, O2> ObservableOutput for MergeObservable<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError, O1, O2> Observable for MergeObservable<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription
	where
		Self: Sized,
	{
		let mut subscription = Subscription::new_empty();

		let rc_subscriber = RcSubscriber::new(destination.upgrade());

		let s1 = self
			.observable_1
			.subscribe(MapIntoSubscriber::new(rc_subscriber.clone()));
		subscription.add(Teardown::Sub(Box::new(s1)));

		let s2 = self
			.observable_2
			.subscribe(MapIntoSubscriber::new(rc_subscriber));
		subscription.add(Teardown::Sub(Box::new(s2)));

		subscription
	}
}
