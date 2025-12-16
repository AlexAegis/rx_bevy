use rx_core_emission_variants::{IntoVariant1of2Subscriber, IntoVariant2of2Subscriber};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, SharedSubscriber, Subscriber, SubscriptionData, TeardownCollection,
	UpgradeableObserver,
};

use crate::{ZipSubscriber, observable::ZipSubscriberOptions};

// TODO: Consider renaming this to Zip2Observable, impl From<(O1, O2)> for it, and impl a new ZipObservable that has an enum inside it across Zip2..Zip3 and impl From<(O1, O2)>
#[derive(RxObservable, Clone, Debug)]
#[rx_out((O1::Out, O2::Out))]
#[rx_out_error(O1::OutError)]
pub struct ZipObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	options: ZipSubscriberOptions,
	observable_1: O1,
	observable_2: O2,
}

impl<O1, O2> ZipObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	pub fn new(o1: O1, o2: O2) -> Self {
		Self {
			options: ZipSubscriberOptions::default(),
			observable_1: o1,
			observable_2: o2,
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
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	type Subscription<Destination>
		= SubscriptionData
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let destination = observer.upgrade();

		let shared_subscriber = SharedSubscriber::new(ZipSubscriber::<_, O1, O2>::new(
			destination,
			self.options.clone(),
		));

		let s1 = self
			.observable_1
			.subscribe(IntoVariant1of2Subscriber::new(shared_subscriber.clone()));

		let s2 = self
			.observable_2
			.subscribe(IntoVariant2of2Subscriber::new(shared_subscriber));

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(s1.into());
		subscription.add_teardown(s2.into());
		subscription
	}
}
