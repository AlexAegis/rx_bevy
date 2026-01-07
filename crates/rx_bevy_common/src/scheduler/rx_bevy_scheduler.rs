use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_scheduler_ticking::{
	SchedulerForTickingExecutor, Tick, TickedContinuousWorkFactory, TickedDelayedOnceWorkFactory,
	TickedImmediateOnceWorkFactory, TickedInvokedWorkFactory, TickedRepeatingWorkFactory,
	TickingScheduler,
};
use rx_core_traits::{
	ScheduledWork, ScheduledWorkAction, Scheduler, WorkCancellationId, WorkInvokeId,
};

use crate::RxBevyContext;

#[derive(Default, Debug, RxScheduler)]
#[rx_tick(Tick)]
#[rx_context(RxBevyContext)]
pub struct RxBevyScheduler {
	ticking_scheduler: TickingScheduler<RxBevyContext>,
}

impl SchedulerForTickingExecutor for RxBevyScheduler {
	#[inline]
	fn drain_actions(&mut self) -> std::vec::Drain<'_, ScheduledWorkAction<Tick, RxBevyContext>> {
		self.ticking_scheduler.drain_actions()
	}

	#[inline]
	fn has_actions(&self) -> bool {
		self.ticking_scheduler.has_actions()
	}

	#[inline]
	fn update_tick(&mut self, tick: Tick) {
		self.ticking_scheduler.update_tick(tick);
	}
}

impl Scheduler for RxBevyScheduler {
	type DelayedWorkFactory = TickedDelayedOnceWorkFactory<RxBevyContext>;
	type ImmediateWorkFactory = TickedImmediateOnceWorkFactory<RxBevyContext>;
	type RepeatedWorkFactory = TickedRepeatingWorkFactory<RxBevyContext>;
	type InvokedWorkFactory = TickedInvokedWorkFactory<RxBevyContext>;
	type ContinuousWorkFactory = TickedContinuousWorkFactory<RxBevyContext>;

	#[inline]
	fn schedule_work<W>(&mut self, work: W, owner_id: WorkCancellationId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
		self.ticking_scheduler.schedule_work(work, owner_id);
	}

	#[inline]
	fn schedule_invoked_work<W>(&mut self, work: W, invoke_id: WorkInvokeId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
		self.ticking_scheduler
			.schedule_invoked_work(work, invoke_id);
	}

	#[inline]
	fn invoke(&mut self, invoke_id: WorkInvokeId) {
		self.ticking_scheduler.invoke(invoke_id);
	}

	#[inline]
	fn cancel(&mut self, owner_id: WorkCancellationId) {
		self.ticking_scheduler.cancel(owner_id);
	}

	#[inline]
	fn cancel_invoked(&mut self, invoke_id: WorkInvokeId) {
		self.ticking_scheduler.cancel_invoked(invoke_id);
	}

	#[inline]
	fn generate_cancellation_id(&mut self) -> WorkCancellationId {
		self.ticking_scheduler.generate_cancellation_id()
	}

	#[inline]
	fn generate_invoke_id(&mut self) -> WorkInvokeId {
		self.ticking_scheduler.generate_invoke_id()
	}
}
