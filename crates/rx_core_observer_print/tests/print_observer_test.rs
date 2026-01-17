use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_complete_when_iterated() {
	let observer = PrintObserver::<usize, Never>::new("iter");
	let mut subscription = (1usize..=3).into_observable().subscribe(observer);
	let teardown = subscription.add_tracked_teardown("print_iter_teardown");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn should_complete_when_of() {
	let observer = PrintObserver::<usize, Never>::new("of");
	let mut subscription = of(5usize).subscribe(observer);
	let teardown = subscription.add_tracked_teardown("print_of_teardown");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn should_error_when_throw() {
	let observer = PrintObserver::<Never, TestError>::new("throw");
	let mut subscription = throw(TestError).subscribe(observer);
	let teardown = subscription.add_tracked_teardown("print_throw_teardown");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut subscription = of(1usize).subscribe(PrintObserver::new("print_contract_complete"));
		let teardown = subscription.add_tracked_teardown("print_contract_complete_teardown");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let mut subscription =
			throw(TestError).subscribe(PrintObserver::new("print_contract_error"));
		let teardown = subscription.add_tracked_teardown("print_contract_error_teardown");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut subscription =
			of(1usize).subscribe(PrintObserver::new("print_contract_unsubscribe"));
		let teardown = subscription.add_tracked_teardown("print_contract_unsubscribe_teardown");

		subscription.unsubscribe();

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}
