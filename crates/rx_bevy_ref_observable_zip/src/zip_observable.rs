use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionCollection,
};
use rx_bevy_emission_variants::{
	EitherOutError2, IntoVariant1of2Subscriber, IntoVariant2of2Subscriber,
};
use rx_bevy_ref_subscriber_rc::RcSubscriber;

use crate::{ZipSubscriber, ZipSubscriberOptions};

pub fn zip<O1, O2>(observable_1: O1, observable_2: O2) -> Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	Zip::new(observable_1, observable_2)
}

pub struct Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	options: ZipSubscriberOptions,
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
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
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context, Subscription = O1::Subscription>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Out = (O1::Out, O2::Out);
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2> SignalContext for Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context, Subscription = O1::Subscription>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context = O1::Context;
}

impl<O1, O2> Observable for Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Context = O1::Context, Subscription = O1::Subscription>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Subscription = O1::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut subscription = Self::Subscription::default();

		let rc_subscriber = RcSubscriber::new(ZipSubscriber::<Destination, O1, O2>::new(
			destination,
			self.options.clone(),
		));

		subscription.add(
			self.observable_1.subscribe(
				IntoVariant1of2Subscriber::new(rc_subscriber.clone()),
				context,
			),
			context,
		);

		subscription.add(
			self.observable_2.subscribe(
				IntoVariant2of2Subscriber::new(rc_subscriber.clone()),
				context,
			),
			context,
		);

		subscription
	}
}
