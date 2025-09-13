use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, TeardownFn,
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
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Out = (O1::Out, O2::Out);
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2> Observable for Zip<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable<Subscription = O1::Subscription>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Subscription = O1::Subscription;

	fn subscribe<'c, Destination>(
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
		let mut subscription = Self::Subscription::default();

		let rc_subscriber = RcSubscriber::new(ZipSubscriber::<Destination, O1, O2>::new(
			destination,
			self.options.clone(),
		));

		let mut sub_1 = self.observable_1.subscribe(
			IntoVariant1of2Subscriber::new(rc_subscriber.clone()),
			context,
		);

		subscription.add(
			TeardownFn::new(move |c| {
				sub_1.unsubscribe(c);
			}),
			context,
		);

		let mut sub_2 = self.observable_2.subscribe(
			IntoVariant2of2Subscriber::new(rc_subscriber.clone()),
			context,
		);

		subscription.add(
			TeardownFn::new(move |c| {
				sub_2.unsubscribe(c);
			}),
			context,
		);

		subscription
	}
}
