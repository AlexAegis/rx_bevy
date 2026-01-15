use rx_core_common::{Never, Observable, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;

/// # [ClosedObservable]
///
/// An observable that immediately closes without completing or emitting any
/// values.
///
/// ## See also:
///
/// - [`empty`]: Completes immediately without emitting any values.
/// - [`never`]: Never emits anything, never closes!
#[derive(RxObservable, Clone, Debug, Default)]
#[rx_out(Never)]
#[rx_out_error(Never)]
pub struct ClosedObservable;

impl Observable for ClosedObservable {
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
		InertSubscription::new(observer.upgrade())
	}
}
