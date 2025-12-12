use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Never, Observable, Subscriber, Subscription, UpgradeableObserver};

/// Never emits anything, never completes.
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

#[cfg(test)]
mod tests {

	use super::*;
	use rx_core::SubscriberNotification;
	use rx_core_testing::MockObserver;
	use rx_core_traits::SubscriptionLike;

	#[test]
	fn should_immediately_emit_complete() {
		let mut observable = NeverObservable;
		let mock_observer = MockObserver::default();
		let notification_collector = mock_observer.get_notification_collector();

		let mut subscription = observable.subscribe(mock_observer);
		subscription.unsubscribe();

		assert!(matches!(
			notification_collector.lock().nth_notification(0),
			SubscriberNotification::Complete
		));
	}
}
