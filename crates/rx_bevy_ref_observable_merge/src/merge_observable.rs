use std::marker::PhantomData;

use rx_bevy_core::{DropSubscription, Observable, ObservableOutput, Teardown, UpgradeableObserver};
use rx_bevy_operator_map_into::MapIntoSubscriber;
use rx_bevy_ref_subscriber_rc::RcSubscriber;

pub fn merge<Out, OutError, O1, O2>(
	observable_1: O1,
	observable_2: O2,
) -> MergeObservable<Out, OutError, O1, O2, ()>
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

pub struct MergeObservable<Out, OutError, O1, O2, Context>
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
	_phantom_data: PhantomData<(Out, OutError, Context)>,
}

impl<Out, OutError, O1, O2, Context> MergeObservable<Out, OutError, O1, O2, Context>
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

impl<Out, OutError, O1, O2, Context> ObservableOutput
	for MergeObservable<Out, OutError, O1, O2, Context>
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

impl<Out, OutError, O1, O2, Context> Observable for MergeObservable<Out, OutError, O1, O2, Context>
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
	type Subscription = DropSubscription<Context>;

	fn subscribe<
		'c,
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination: Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
		Self: Sized,
	{
		let mut subscription = DropSubscription::new_empty();

		let rc_subscriber = RcSubscriber::new(destination.upgrade());

		let s1 = self
			.observable_1
			.subscribe(MapIntoSubscriber::new(rc_subscriber.clone()), context);
		subscription.add(Teardown::Sub(Box::new(s1)), context);

		let s2 = self
			.observable_2
			.subscribe(MapIntoSubscriber::new(rc_subscriber), context);
		subscription.add(Teardown::Sub(Box::new(s2)), context);

		subscription
	}
}
