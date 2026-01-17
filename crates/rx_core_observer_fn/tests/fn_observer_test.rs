use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

fn setup() -> (
	FnObserver<usize, &'static str, impl FnMut(usize), impl FnOnce(&'static str), impl FnOnce()>,
	Arc<Mutex<Vec<usize>>>,
	Arc<Mutex<Option<&'static str>>>,
	Arc<AtomicBool>,
) {
	let next_buffer = Arc::new(Mutex::new(Vec::<usize>::new()));
	let error_buffer = Arc::new(Mutex::new(Option::<&'static str>::None));
	let completion_buffer = Arc::new(AtomicBool::new(false));

	let next_buffer_for_observer = next_buffer.clone();
	let error_buffer_for_observer = error_buffer.clone();
	let completion_buffer_for_observer = completion_buffer.clone();
	let fn_observer = FnObserver::new(
		move |next| {
			next_buffer_for_observer.lock_ignore_poison().push(next);
		},
		move |error| {
			error_buffer_for_observer
				.lock_ignore_poison()
				.replace(error);
		},
		move || {
			completion_buffer_for_observer.store(true, Ordering::Relaxed);
		},
	);

	(fn_observer, next_buffer, error_buffer, completion_buffer)
}

#[test]
fn should_call_the_next_fn_when_nexted_into() {
	let (mut fn_observer, next_buffer, _error_buffer, _completion_buffer) = setup();
	fn_observer.next(1);

	assert_eq!(*next_buffer.lock_ignore_poison().first().unwrap(), 1)
}

#[test]
fn should_call_the_error_fn_when_errored() {
	let (mut fn_observer, _next_buffer, error_buffer, _completion_buffer) = setup();
	let error = "error";
	fn_observer.error(error);

	assert_eq!(error_buffer.lock_ignore_poison().unwrap(), error)
}

#[test]
fn should_call_the_complete_fn_when_completed() {
	let (mut fn_observer, _next_buffer, _error_buffer, completion_buffer) = setup();
	assert!(!completion_buffer.load(Ordering::Relaxed));
	fn_observer.complete();
	assert!(completion_buffer.load(Ordering::Relaxed))
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let (teardown_complete, tracker_complete) =
			Teardown::tracked("fn_observer_contract_complete_callback");
		let mut subscription = just(1usize).subscribe(FnObserver::new(
			|_next: usize| {},
			|_error: Never| {},
			move || {
				teardown_complete.execute();
			},
		));
		let teardown = subscription.add_tracked_teardown("fn_observer_contract_complete");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
		tracker_complete.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let (teardown_error, tracker_error) =
			Teardown::tracked("fn_observer_contract_error_callback");
		let mut subscription = throw(TestError).subscribe(FnObserver::new(
			|_next: Never| {},
			move |error: TestError| {
				assert_eq!(error, TestError);
				teardown_error.execute();
			},
			|| {},
		));
		let teardown = subscription.add_tracked_teardown("fn_observer_contract_error");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
		tracker_error.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut subscription = never().subscribe(FnObserver::new(
			|_next: Never| {},
			|_error: Never| {},
			move || {},
		));
		let teardown = subscription.add_tracked_teardown("fn_observer_contract_unsubscribe");

		subscription.unsubscribe();

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}
