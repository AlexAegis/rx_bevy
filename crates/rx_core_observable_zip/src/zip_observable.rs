use rx_core_emission_variants::{
	EitherOutError2, IntoVariant1of2Subscriber, IntoVariant2of2Subscriber,
};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Subscriber, SubscriptionContext, SubscriptionData, UpgradeableObserver,
};

use crate::{ZipSubscriber, observable::ZipSubscriberOptions};

#[derive(RxObservable, Clone, Debug)]
#[rx_out((O1::Out, O2::Out))]
#[rx_out_error(EitherOutError2<O1, O2>)]
#[rx_context(O1::Context)]
pub struct ZipObservable<O1, O2>
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

impl<O1, O2> ZipObservable<O1, O2>
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

impl<O1, O2> Observable for ZipObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Subscription<Destination>
		= SubscriptionData<O1::Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let destination = observer.upgrade();
		let rc_subscriber = RcSubscriber::new(
			ZipSubscriber::<_, O1, O2>::new(destination, self.options.clone()),
			context,
		);

		let s1 = self.observable_1.subscribe(
			IntoVariant1of2Subscriber::new(rc_subscriber.clone_with_context(context)),
			context,
		);

		let s2 = self.observable_2.subscribe(
			IntoVariant2of2Subscriber::new(rc_subscriber.clone_with_context(context)),
			context,
		);

		let mut subscription = SubscriptionData::default();
		subscription.add_notifiable(s1.into(), context);
		subscription.add_notifiable(s2.into(), context);
		subscription
	}
}
