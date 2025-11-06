use core::marker::PhantomData;

use rx_core_operator_map_into::MapIntoSubscriber;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, ObservableOutput, PrimaryCategoryObservable, SignalBound, SubscriptionContext,
	SubscriptionData, UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

pub struct MergeObservable<Out, OutError, O1, O2>
where
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	observable_1: O1,
	observable_2: O2,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, O1, O2> MergeObservable<Out, OutError, O1, O2>
where
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
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
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError, O1, O2> WithSubscriptionContext for MergeObservable<Out, OutError, O1, O2>
where
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	type Context = O1::Context;
}

impl<Out, OutError, O1, O2> WithPrimaryCategory for MergeObservable<Out, OutError, O1, O2>
where
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<Out, OutError, O1, O2> Observable for MergeObservable<Out, OutError, O1, O2>
where
	Out: SignalBound,
	OutError: SignalBound,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	<O1 as Observable>::Subscription: 'static,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
	<O2 as Observable>::Subscription: 'static,
{
	type Subscription = SubscriptionData<O1::Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let destination = observer.upgrade();
		let rc_subscriber = RcSubscriber::new(destination, context);

		let s1 = self.observable_1.subscribe(
			MapIntoSubscriber::new(rc_subscriber.clone_with_context(context)),
			context,
		);

		let s2 = self
			.observable_2
			.subscribe(MapIntoSubscriber::new(rc_subscriber), context);

		let mut subscription = SubscriptionData::default();
		subscription.add_notifiable(s1.into(), context);
		subscription.add_notifiable(s2.into(), context);
		subscription
	}
}
