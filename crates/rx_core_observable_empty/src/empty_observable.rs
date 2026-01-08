use rx_core_common::{Never, Observable, RxObserver, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;

/// # [EmptyObservable]
///
/// Immediately completes without emitting anything.
#[derive(RxObservable, Clone, Debug, Default)]
#[rx_out(Never)]
#[rx_out_error(Never)]
pub struct EmptyObservable;

impl Observable for EmptyObservable {
	type Subscription<Destination>
		= InertSubscription
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
		let mut destination = observer.upgrade();
		destination.complete();
		InertSubscription::new(destination)
	}
}
