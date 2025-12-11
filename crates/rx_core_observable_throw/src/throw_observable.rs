use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{Never, Observable, Observer, Signal, Subscriber, UpgradeableObserver};

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
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut destination = observer.upgrade();
		destination.error(self.error.clone());
		InertSubscription::new(destination)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_core_testing::prelude::*;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mock_observer = MockObserver::default();
		let notification_collector = mock_observer.get_notification_collector();

		let _s = observable.subscribe(mock_observer);

		assert_eq!(
			notification_collector.lock().all_observed_errors(),
			vec![error]
		);
	}
}
