use rx_core::prelude::*;

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
