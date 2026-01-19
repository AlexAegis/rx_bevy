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

#[test]
fn default_observer_completes() {
	let mut subscription = just(42usize).subscribe(PrintObserver::<usize, Never>::default());
	let teardown = subscription.add_tracked_teardown("print default");

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn clone_unsubscribe_does_not_close_original() {
	let mut subject = PublishSubject::<usize, Never>::default();
	let observer = PrintObserver::<usize, Never>::new("print");

	let mut original_subscription = subject.clone().subscribe(observer.clone());
	let original_teardown = original_subscription.add_tracked_teardown("print original");

	let mut cloned_subscription = subject.subscribe(observer);
	let cloned_teardown = cloned_subscription.add_tracked_teardown("print clone");

	cloned_subscription.unsubscribe();
	assert!(cloned_subscription.is_closed());
	assert!(!original_subscription.is_closed());

	subject.next(1);
	assert!(!original_subscription.is_closed());

	original_subscription.unsubscribe();
	original_teardown.assert_was_torn_down();
	cloned_teardown.assert_was_torn_down();
	assert!(original_subscription.is_closed());
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
		let mut subject = PublishSubject::<usize, MockError>::default();
		let mut subscription = subject.subscribe(PrintObserver::new("print"));
		let teardown = subscription.add_tracked_teardown("print");

		subject.next(1);
		subscription.unsubscribe();

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}
