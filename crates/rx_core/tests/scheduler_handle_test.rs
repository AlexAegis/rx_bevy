use std::thread;

use rx_core::prelude::*;
use rx_core_scheduler_ticking::TickingScheduler;

struct TestContextProvider;

impl WorkContextProvider for TestContextProvider {
	type Item<'c> = TestContext;
}

#[derive(Default)]
struct TestContext;

impl WorkContext<'_> for TestContext {}

mod poison {
	use super::*;

	#[test]
	fn recovers_from_poison() {
		let handle = SchedulerHandle::new(TickingScheduler::<TestContextProvider>::default());
		let poisoned = handle.get_scheduler_handle();

		let join = thread::spawn(move || {
			let _guard = poisoned.lock();
			panic!("poisoning the scheduler");
		});

		assert!(join.join().is_err());

		let mut guard = handle.lock();
		assert_eq!(*guard.generate_cancellation_id(), 0);
		assert_eq!(*guard.generate_invoke_id(), 0);
	}
}

mod conversion {
	use super::*;

	#[test]
	fn it_should_create_a_handle() {
		let handle = TickingScheduler::<TestContextProvider>::default().into_handle();
		let cloned = handle.get_scheduler_handle();

		let first = {
			let mut guard = handle.lock();
			guard.generate_invoke_id()
		};

		let second = {
			let mut guard = cloned.lock();
			guard.generate_invoke_id()
		};

		assert_eq!(*first, 0);
		assert_eq!(*second, 1);
	}
}
