use rx_core::{SubscriberNotification, prelude::EmptyObservable};
use rx_core_testing::MockObserver;
use rx_core_traits::prelude::*;

#[test]
fn should_immediately_emit_complete() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = EmptyObservable.subscribe(mock_observer);
	subscription.unsubscribe();

	assert!(matches!(
		notification_collector.lock().nth_notification(0),
		SubscriberNotification::Complete
	));
}

mod observable_fn {
	use rx_core::prelude::empty;

	use super::*;

	#[test]
	fn should_immediately_emit_complete() {
		let mock_observer = MockObserver::default();
		let notification_collector = mock_observer.get_notification_collector();

		let mut subscription = empty().subscribe(mock_observer);
		subscription.unsubscribe();

		assert!(matches!(
			notification_collector.lock().nth_notification(0),
			SubscriberNotification::Complete
		));
	}
}
