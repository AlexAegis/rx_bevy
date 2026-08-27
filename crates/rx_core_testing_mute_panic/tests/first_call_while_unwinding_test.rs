use std::panic::{catch_unwind, set_hook};

use rx_core_testing_mute_panic::mute_panic;

struct MuteWhileDropping;

impl Drop for MuteWhileDropping {
	fn drop(&mut self) {
		mute_panic(|| ());
	}
}

#[test]
fn should_survive_a_first_call_from_a_drop_during_unwinding() {
	set_hook(Box::new(|_| {}));

	let caught = catch_unwind(|| {
		let _mute_while_dropping = MuteWhileDropping;
		panic!("expected");
	});

	assert!(caught.is_err());
	assert!(mute_panic(|| true));
}
