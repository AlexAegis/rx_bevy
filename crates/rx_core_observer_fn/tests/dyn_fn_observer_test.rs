use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_testing::mute_panic;

fn setup() -> (
	DynFnObserver<usize, &'static str>,
	impl Fn(usize),
	impl FnOnce(&'static str),
	impl FnOnce(),
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

	let next_fn = move |next| {
		next_buffer_for_observer.lock_ignore_poison().push(next);
	};
	let error_fn = move |error| {
		error_buffer_for_observer
			.lock_ignore_poison()
			.replace(error);
	};
	let complete_fn = move || {
		completion_buffer_for_observer.store(true, Ordering::Relaxed);
	};
	let dyn_fn_observer = DynFnObserver::default();

	(
		dyn_fn_observer,
		next_fn,
		error_fn,
		complete_fn,
		next_buffer,
		error_buffer,
		completion_buffer,
	)
}

#[test]
fn should_call_the_next_fn_when_nexted_into() {
	let (
		dyn_fn_observer,
		next_fn,
		_error_fn,
		_complete_fn,
		next_buffer,
		_error_buffer,
		_completion_buffer,
	) = setup();
	let mut dyn_fn_observer = dyn_fn_observer.with_next(next_fn);
	dyn_fn_observer.next(1);

	assert_eq!(*next_buffer.lock_ignore_poison().first().unwrap(), 1)
}

#[test]
fn should_call_the_error_fn_when_errored() {
	let (
		dyn_fn_observer,
		_next_fn,
		error_fn,
		_complete_fn,
		_next_buffer,
		error_buffer,
		_completion_buffer,
	) = setup();
	let error = "error";
	let mut dyn_fn_observer = dyn_fn_observer.with_error(error_fn);
	dyn_fn_observer.error(error);

	assert_eq!(error_buffer.lock_ignore_poison().unwrap(), error)
}

#[test]
#[should_panic]
fn should_panic_when_errored_without_an_error_fn() {
	let (
		mut dyn_fn_observer,
		_next_fn,
		_error_fn,
		_complete_fn,
		_next_buffer,
		_error_buffer,
		_completion_buffer,
	) = setup();
	let error = "error";
	mute_panic(|| dyn_fn_observer.error(error));
}

#[test]
fn should_call_the_complete_fn_when_completed() {
	let (
		dyn_fn_observer,
		_next_fn,
		_error_fn,
		complete_fn,
		_next_buffer,
		_error_buffer,
		completion_buffer,
	) = setup();
	let mut dyn_fn_observer = dyn_fn_observer.with_complete(complete_fn);
	assert!(!completion_buffer.load(Ordering::Relaxed));
	dyn_fn_observer.complete();
	assert!(completion_buffer.load(Ordering::Relaxed))
}
