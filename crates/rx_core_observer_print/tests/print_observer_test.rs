use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_complete_with_iterator() {
	let mut subscription = (1usize..=3)
		.into_observable()
		.subscribe(PrintObserver::<usize, Never>::new("print"));
	let teardown = subscription.add_tracked_teardown("print");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn should_complete_with_just() {
	let mut subscription = just(5usize).subscribe(PrintObserver::<usize, Never>::new("print"));
	let teardown = subscription.add_tracked_teardown("print");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn should_error_with_throw() {
	let mut subscription =
		throw(MockError).subscribe(PrintObserver::<Never, MockError>::new("print"));
	let teardown = subscription.add_tracked_teardown("print");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut subscription = just(1usize).subscribe(PrintObserver::new("print"));
		let teardown = subscription.add_tracked_teardown("print");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let mut subscription = throw(MockError).subscribe(PrintObserver::new("print"));
		let teardown = subscription.add_tracked_teardown("print");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut subscription = just(1usize).subscribe(PrintObserver::new("print"));
		let teardown = subscription.add_tracked_teardown("print");

		subscription.unsubscribe();

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}
