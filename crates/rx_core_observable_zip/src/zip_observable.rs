use rx_core_traits::{
	Observable, ObservableOutput, Subscriber, SubscriptionData, context::WithSubscriptionContext,
	prelude::SubscriptionContext,
};
use rx_core_emission_variants::{
	EitherOutError2, IntoVariant1of2Subscriber, IntoVariant2of2Subscriber,
};
use rx_core_subscriber_rc::RcSubscriber;

use crate::{ZipSubscriber, ZipSubscriberOptions};

pub fn zip<O1, O2>(observable_1: O1, observable_2: O2) -> Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	Zip::new(observable_1, observable_2)
}

pub struct Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	options: ZipSubscriberOptions,
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(observable_1: O1, observable_2: O2) -> Self {
		Self {
			options: ZipSubscriberOptions::default(),
			observable_1,
			observable_2,
		}
	}

	pub fn with_options(mut self, options: ZipSubscriberOptions) -> Self {
		self.options = options;
		self
	}
}

impl<O1, O2> ObservableOutput for Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Out = (O1::Out, O2::Out);
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2> WithSubscriptionContext for Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context = O1::Context;
}

impl<O1, O2> Observable for Zip<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Subscription = SubscriptionData<O1::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let rc_subscriber = RcSubscriber::new(
			ZipSubscriber::<Destination, O1, O2>::new(destination, self.options.clone()),
			context,
		);

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
		subscription
	}
}
