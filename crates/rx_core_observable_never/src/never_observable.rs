use rx_core_common::{Never, Observable, Subscriber, Subscription, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;

/// Never emits anything, never completes and most importantly, it
/// Never unsubscribes!
#[derive(RxObservable, Clone, Debug, Default)]
#[rx_out(Never)]
#[rx_out_error(Never)]
pub struct NeverObservable;

impl Observable for NeverObservable {
	type Subscription<Destination>
		= Subscription<Destination>
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
		Subscription::new(observer.upgrade())
	}
}
