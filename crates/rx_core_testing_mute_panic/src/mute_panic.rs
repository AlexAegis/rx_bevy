use std::{cell::Cell, panic, sync::Once};

thread_local! {
	static MUTED: Cell<bool> = const { Cell::new(false) };
}

static INSTALLED: Once = Once::new();

fn install_hook() {
	// take_hook panics if this thread is already unwinding.
	if std::thread::panicking() {
		return;
	}

	INSTALLED.call_once(|| {
		let previous = panic::take_hook();
		panic::set_hook(Box::new(move |info| {
			if !MUTED.with(Cell::get) {
				previous(info);
			}
		}));
	});
}

struct MuteGuard {
	previous: bool,
}

impl Drop for MuteGuard {
	fn drop(&mut self) {
		MUTED.with(|muted| muted.set(self.previous));
	}
}

/// Silences the panic message while `fun` runs, on the calling thread only.
/// Useful for `#[should_panic]` tests run with `--nocapture`.
pub fn mute_panic<R>(fun: impl FnOnce() -> R) -> R {
	install_hook();
	let _guard = MuteGuard {
		previous: MUTED.with(|muted| muted.replace(true)),
	};
	fun()
}

#[cfg(test)]
mod test {
	use super::*;
	use std::panic::catch_unwind;

	fn is_muted() -> bool {
		MUTED.with(Cell::get)
	}

	#[test]
	fn should_return_what_the_closure_returns() {
		assert_eq!(mute_panic(|| 7), 7);
	}

	#[test]
	fn should_unmute_after_the_closure_returns() {
		mute_panic(|| assert!(is_muted()));
		assert!(!is_muted());
	}

	#[test]
	fn should_unmute_after_the_closure_panics() {
		let caught = catch_unwind(|| mute_panic(|| panic!("expected")));
		assert!(caught.is_err());
		assert!(!is_muted());
	}

	#[test]
	fn should_stay_muted_after_a_nested_call_returns() {
		mute_panic(|| {
			mute_panic(|| assert!(is_muted()));
			assert!(is_muted());
		});
		assert!(!is_muted());
	}

	#[test]
	fn should_not_mute_another_thread() {
		mute_panic(|| {
			let other = std::thread::spawn(is_muted);
			assert!(!other.join().expect("the other thread should not panic"));
		});
	}
}
