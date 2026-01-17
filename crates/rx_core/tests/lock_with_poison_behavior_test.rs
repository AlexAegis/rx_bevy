use std::{
	panic,
	sync::{
		Arc, Mutex, RwLock,
		atomic::{AtomicBool, Ordering},
	},
};

use rx_core_common::{
	LockWithPoisonBehavior, ReadLockWithPoisonBehavior, WriteLockWithPoisonBehavior,
};
use rx_core_testing::prelude::*;

#[test]
fn lock_with_poison_behavior_runs_handler_for_mutex() {
	let shared = Arc::new(Mutex::new(Vec::new()));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.lock().unwrap();
			guard.push("before");
			mute_panic(|| panic!("poison"));
		});
	}

	let mut guard = shared.lock_with_poison_behavior(|inner| inner.push("handled"));
	guard.push("after");
	drop(guard);

	let guard = shared.lock_ignore_poison();
	assert_eq!(guard.as_slice(), ["before", "handled", "after"]);
}

#[test]
fn lock_ignore_then_clear_controls_mutex_poison_flag() {
	let shared = Arc::new(Mutex::new(0usize));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.lock().unwrap();
			*guard = 1;
			mute_panic(|| panic!("poison"));
		});
	}

	{
		let mut guard = shared.lock_ignore_poison();
		*guard += 1;
	}

	assert!(shared.lock().is_err());

	{
		let mut guard = shared.lock_clear_poison();
		*guard += 1;
	}

	let guard = shared.lock().unwrap();
	assert_eq!(*guard, 3);
}

#[test]
fn write_lock_with_poison_behavior_recovers_write_access() {
	let shared = Arc::new(RwLock::new(Vec::<usize>::new()));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.write().unwrap();
			guard.push(1);
			mute_panic(|| panic!("poison"));
		});
	}

	let mut guard = shared.write_lock_with_poison_behavior(|inner| inner.push(2));
	guard.push(3);
	drop(guard);

	let guard = shared.write_lock_ignore_poison();
	assert_eq!(guard.as_slice(), [1, 2, 3]);
}

#[test]
fn write_lock_ignore_then_clear_controls_rw_lock_poison_flag() {
	let shared = Arc::new(RwLock::new(1usize));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.write().unwrap();
			*guard = 2;
			mute_panic(|| panic!("poison"));
		});
	}

	{
		let mut guard = shared.write_lock_ignore_poison();
		*guard += 1;
	}

	assert!(shared.write().is_err());

	{
		let mut guard = shared.write_lock_clear_poison();
		*guard += 1;
	}

	let guard = shared.read().unwrap();
	assert_eq!(*guard, 4);
}

#[test]
fn read_lock_with_poison_behavior_invokes_handler() {
	let shared = Arc::new(RwLock::new(5usize));
	let handler_called = Arc::new(AtomicBool::new(false));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.write().unwrap();
			*guard = 6;
			mute_panic(|| panic!("poison"));
		});
	}

	let handler_called_clone = handler_called.clone();
	let guard = shared.read_lock_with_poison_behavior(|_| {
		handler_called_clone.store(true, Ordering::Relaxed);
	});

	assert!(handler_called.load(Ordering::Relaxed));
	assert_eq!(*guard, 6);
}

#[test]
fn read_lock_ignore_then_clear_controls_rw_lock_poison_flag() {
	let shared = Arc::new(RwLock::new(10usize));

	{
		let poisoned = shared.clone();
		let _ = panic::catch_unwind(|| {
			let mut guard = poisoned.write().unwrap();
			*guard = 11;
			mute_panic(|| panic!("poison"));
		});
	}

	{
		let guard = shared.read_lock_ignore_poison();
		assert_eq!(*guard, 11);
	}

	assert!(shared.read().is_err());

	{
		let guard = shared.read_lock_clear_poison();
		assert_eq!(*guard, 11);
	}

	let guard = shared.read().unwrap();
	assert_eq!(*guard, 11);
}
