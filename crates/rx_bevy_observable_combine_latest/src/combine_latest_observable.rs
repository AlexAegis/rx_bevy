use rx_bevy_observable::{
	Observable, ObservableOutput, RcSubscriber, Subscription, UpgradeableObserver,
};

use crate::{EitherEmission, EitherError, InnerCombinatorSubscriber, IntoVariantSubscriber};

pub fn combine_latest<O1, O2>(observable_1: O1, observable_2: O2) -> CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	CombineLatest::new(observable_1, observable_2)
}

pub struct CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
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

impl<O1, O2> ObservableOutput for CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Out = (O1::Out, O2::Out);
	type OutError = EitherError<O1, O2>;
}

impl<O1, O2> Observable for CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
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
		let upgraded_subscriber = destination.upgrade();

		let s =
			InnerCombinatorSubscriber::<Destination::Subscriber, O1, O2>::new(upgraded_subscriber);

		let rc_subscriber = RcSubscriber::new(s);

		let s1 = self.observable_1.subscribe(IntoVariantSubscriber::new(
			rc_subscriber.clone(),
			|inp| EitherEmission::O1(inp),
			|in_error| EitherError::O1Error(in_error),
		));
		subscription.add(s1);

		let s2 = self.observable_2.subscribe(IntoVariantSubscriber::new(
			rc_subscriber.clone(),
			|inp| EitherEmission::O2(inp),
			|in_error| EitherError::O2Error(in_error),
		));
		subscription.add(s2);

		subscription
	}
}
