use std::{
	panic::{catch_unwind, set_hook},
	sync::atomic::{AtomicUsize, Ordering},
};

use rx_core_testing_mute_panic::mute_panic;

static REPORTED: AtomicUsize = AtomicUsize::new(0);

fn reported() -> usize {
	REPORTED.load(Ordering::Relaxed)
}

#[test]
fn should_report_every_panic_except_the_muted_one_as_the_only_test_in_this_binary() {
	set_hook(Box::new(|info| {
		REPORTED.fetch_add(1, Ordering::Relaxed);
		eprintln!("{info}");
	}));

	let muted = catch_unwind(|| mute_panic(|| panic!("muted")));
	assert!(muted.is_err());
	assert_eq!(reported(), 0);

	let unmuted = catch_unwind(|| panic!("unmuted"));
	assert!(unmuted.is_err());
	assert_eq!(reported(), 1);
}
