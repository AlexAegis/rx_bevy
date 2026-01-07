use rx_core_common::{
	ScheduledWork, Scheduler, SchedulerHandle, WorkCancellationId, WorkContext,
	WorkContextProvider, WorkInvokeId,
};
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_scheduler_ticking::{
	Tick, TickedContinuousWorkFactory, TickedDelayedOnceWorkFactory,
	TickedImmediateOnceWorkFactory, TickedInvokedWorkFactory, TickedRepeatingWorkFactory,
};

#[derive(RxScheduler)]
#[rx_context(FakeContext)]
#[rx_tick(Tick)]
struct FakeScheduler;

impl Scheduler for FakeScheduler {
	type DelayedWorkFactory = TickedDelayedOnceWorkFactory<FakeContext>;
	type ImmediateWorkFactory = TickedImmediateOnceWorkFactory<FakeContext>;
	type RepeatedWorkFactory = TickedRepeatingWorkFactory<FakeContext>;
	type InvokedWorkFactory = TickedInvokedWorkFactory<FakeContext>;
	type ContinuousWorkFactory = TickedContinuousWorkFactory<FakeContext>;

	fn cancel(&mut self, _cancellation_id: WorkCancellationId) {}

	fn cancel_invoked(&mut self, _invoke_id: WorkInvokeId) {}

	fn generate_cancellation_id(&mut self) -> WorkCancellationId {
		unreachable!()
	}

	fn generate_invoke_id(&mut self) -> WorkInvokeId {
		unreachable!()
	}

	fn invoke(&mut self, _invoke_id: WorkInvokeId) {}

	fn schedule_invoked_work<W>(&mut self, _work: W, _invoke_id: WorkInvokeId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
	}
	fn schedule_work<W>(&mut self, _work: W, _cancellation_id: WorkCancellationId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
	}
}

struct FakeWorkContext;

impl<'a> WorkContext<'a> for FakeWorkContext {}

struct FakeContext;

impl WorkContextProvider for FakeContext {
	type Item<'c> = FakeWorkContext;
}

#[derive(RxExecutor)]
#[rx_context(FakeContext)]
#[rx_tick(Tick)]
#[rx_scheduler(FakeScheduler)]
struct FakeExecutor {
	#[scheduler_handle]
	scheduler: SchedulerHandle<FakeScheduler>,
}

#[test]
fn compiles() {
	let fake_scheduler = FakeScheduler;
	let _fake_executor = FakeExecutor {
		scheduler: SchedulerHandle::new(fake_scheduler),
	};
}
