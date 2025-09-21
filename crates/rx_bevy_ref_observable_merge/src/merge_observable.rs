use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionCollection,
};
use rx_bevy_operator_map_into::MapIntoSubscriber;
use rx_bevy_ref_subscriber_rc::RcSubscriber;

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
	O2: Observable<Subscription = O1::Subscription>,
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
	O2: Observable<Subscription = O1::Subscription>,
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
	O2: Observable<Subscription = O1::Subscription>,
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
	O2: Observable<Subscription = O1::Subscription>,
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
	<O1 as Observable>::Subscription: 'static,
	O2: Observable<Subscription = O1::Subscription>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
	<O2 as Observable>::Subscription: 'static,
{
	type Subscription = O1::Subscription;

	fn subscribe<
		'c,
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Self: Sized,
	{
		let mut subscription = O1::Subscription::default();

		let rc_subscriber = RcSubscriber::new(destination);

		subscription.add(
			self.observable_1
				.subscribe(MapIntoSubscriber::new(rc_subscriber.clone()), context),
			context,
		);

		subscription.add(
			self.observable_2
				.subscribe(MapIntoSubscriber::new(rc_subscriber), context),
			context,
		);

		subscription
	}
}
