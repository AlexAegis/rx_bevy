use rx_core_common::{Never, Observable, RxObserver, Signal, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;

#[derive(RxObservable, Clone, Debug)]
#[rx_out(Never)]
#[rx_out_error(OutError)]
pub struct ThrowObservable<OutError>
where
	OutError: Signal + Clone,
{
	error: OutError,
}

impl<OutError> ThrowObservable<OutError>
where
	OutError: Signal + Clone,
{
	pub fn new(error: OutError) -> Self {
		Self { error }
	}
}

impl<OutError> Observable for ThrowObservable<OutError>
where
	OutError: Signal + Clone,
{
	type Subscription<Destination>
		= InertSubscription
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut subscriber = destination.upgrade();
		subscriber.error(self.error.clone());
		InertSubscription::new(subscriber)
	}
}
