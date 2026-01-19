use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn nothing_should_happen_when_nexted() {
	let mut noop_observer = NoopObserver::<usize, &'static str>::default();

	noop_observer.next(1);
}

#[test]
fn nothing_should_happen_when_completed() {
	let mut noop_observer = NoopObserver::<usize, &'static str>::default();

	noop_observer.complete();
}

#[test]
#[should_panic]
fn should_panic_when_errored_in_dev_mode() {
	use rx_core_testing::mute_panic;

	let mut noop_observer = NoopObserver::<usize, &'static str>::default();

	mute_panic(|| noop_observer.error("error"));
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut subscription = just(1usize).subscribe(NoopObserver::default());
		let teardown = subscription.add_tracked_teardown("noop_contract_complete");

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	#[should_panic]
	fn rx_contract_closed_after_error() {
		mute_panic(move || {
			let mut subscription = throw(MockError).subscribe(NoopObserver::default());
			let teardown = subscription.add_tracked_teardown("noop_contract_error");

			teardown.assert_was_torn_down();
			assert!(subscription.is_closed());
		});
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut subscription = just(1usize).subscribe(NoopObserver::default());
		let teardown = subscription.add_tracked_teardown("noop_contract_unsubscribe");

		subscription.unsubscribe();

		teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}
