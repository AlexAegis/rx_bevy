use rx_core_common::{Never, Observable, RxObserver, Signal, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;

/// # OfObservable
///
/// Emits a single value then immediately completes
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Out)]
#[rx_out_error(Never)]
pub struct OfObservable<Out>
where
	Out: Signal + Clone,
{
	value: Out,
}

impl<Out> OfObservable<Out>
where
	Out: Signal + Clone,
{
	pub fn new(value: Out) -> Self {
		Self { value }
	}
}

impl<Out> Observable for OfObservable<Out>
where
	Out: Signal + Clone,
{
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
		destination.next(self.value.clone());
		destination.complete();
		InertSubscription::new(destination)
	}
}
