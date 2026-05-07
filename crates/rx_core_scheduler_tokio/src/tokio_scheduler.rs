use rx_core_common::{
	ScheduledWork, ScheduledWorkAction, Scheduler, WorkCancellationId, WorkInvokeId,
};
use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_scheduler_ticking::{
	SchedulerForTickingExecutor, Tick, TickedContinuousWorkFactory, TickedDelayedOnceWorkFactory,
	TickedImmediateOnceWorkFactory, TickedInvokedWorkFactory, TickedRepeatingWorkFactory,
	TickingScheduler,
};

use crate::UnitContext;

#[derive(Default, Debug, RxScheduler)]
#[rx_tick(Tick)]
#[rx_context(UnitContext)]
pub struct TokioScheduler {
	ticking_scheduler: TickingScheduler<UnitContext>,
}

impl SchedulerForTickingExecutor for TokioScheduler {
	#[inline]
	fn drain_actions(&mut self) -> std::vec::Drain<'_, ScheduledWorkAction<Tick, UnitContext>> {
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

impl Scheduler for TokioScheduler {
	type DelayedWorkFactory = TickedDelayedOnceWorkFactory<UnitContext>;
	type ImmediateWorkFactory = TickedImmediateOnceWorkFactory<UnitContext>;
	type RepeatedWorkFactory = TickedRepeatingWorkFactory<UnitContext>;
	type InvokedWorkFactory = TickedInvokedWorkFactory<UnitContext>;
	type ContinuousWorkFactory = TickedContinuousWorkFactory<UnitContext>;

	#[inline]
	fn schedule_work<W>(&mut self, work: W, cancellation_id: WorkCancellationId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
		self.ticking_scheduler.schedule_work(work, cancellation_id);
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
	fn cancel(&mut self, cancellation_id: WorkCancellationId) {
		self.ticking_scheduler.cancel(cancellation_id);
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
