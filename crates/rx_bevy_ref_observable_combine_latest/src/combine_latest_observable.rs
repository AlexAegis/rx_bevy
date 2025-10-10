use rx_bevy_core::{
	Observable, ObservableOutput, Subscriber, SubscriptionData, SubscriptionHandle, WithContext,
};
use rx_bevy_emission_variants::{
	EitherOutError2, IntoVariant1of2Subscriber, IntoVariant2of2Subscriber,
};
use rx_bevy_ref_subscriber_rc::RcSubscriber;

use crate::CombineLatestSubscriber;

pub fn combine_latest<O1, O2>(observable_1: O1, observable_2: O2) -> CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	CombineLatest::new(observable_1, observable_2)
}

pub struct CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context>,
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
	O2: 'static + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Out = (O1::Out, O2::Out);
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2> WithContext for CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context = O1::Context;
}

impl<O1, O2> Observable for CombineLatest<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Subscription = SubscriptionData<O1::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let rc_subscriber = RcSubscriber::new(CombineLatestSubscriber::<Destination, O1, O2>::new(
			destination,
		));

		let s1 = self.observable_1.subscribe(
			IntoVariant1of2Subscriber::new(rc_subscriber.clone()),
			context,
		);

		let s2 = self.observable_2.subscribe(
			IntoVariant2of2Subscriber::new(rc_subscriber.clone()),
			context,
		);

		let mut subscription = SubscriptionData::default();
		subscription.add_notifiable(s1.into(), context);
		subscription.add_notifiable(s2.into(), context);
		SubscriptionHandle::new(subscription)
	}
}
