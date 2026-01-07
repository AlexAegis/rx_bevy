use derive_where::derive_where;
use rx_core_common::{
	ScheduledWork, ScheduledWorkAction, Scheduler, WorkCancellationId, WorkCancellationIdGenerator,
	WorkContextProvider, WorkInvokeId, WorkInvokeIdGenerator,
};
use rx_core_macro_scheduler_derive::RxScheduler;

use crate::{
	SchedulerForTickingExecutor, Tick, TickedContinuousWorkFactory, TickedDelayedOnceWorkFactory,
	TickedImmediateOnceWorkFactory, TickedInvokedWorkFactory, TickedRepeatingWorkFactory,
};

#[derive(RxScheduler)]
#[derive_where(Default, Debug)]
#[rx_tick(Tick)]
#[rx_context(C)]
pub struct TickingScheduler<C>
where
	C: WorkContextProvider,
{
	cancellation_id_generator: WorkCancellationIdGenerator,
	invoke_id_generator: WorkInvokeIdGenerator,
	/// Updated by the executor at the start of each tick.
	pub(crate) current_tick: Tick,
	action_queue: Vec<ScheduledWorkAction<Tick, C>>,
}

impl<C> SchedulerForTickingExecutor for TickingScheduler<C>
where
	C: 'static + WorkContextProvider + Send + Sync,
{
	#[inline]
	fn drain_actions(&mut self) -> std::vec::Drain<'_, ScheduledWorkAction<Tick, C>> {
		self.action_queue.drain(..)
	}

	#[inline]
	fn has_actions(&self) -> bool {
		!self.action_queue.is_empty()
	}

	#[inline]
	fn update_tick(&mut self, tick: Tick) {
		self.current_tick.update(tick);
	}
}

impl<C> Scheduler for TickingScheduler<C>
where
	C: 'static + WorkContextProvider + Send + Sync,
{
	type DelayedWorkFactory = TickedDelayedOnceWorkFactory<C>;
	type ImmediateWorkFactory = TickedImmediateOnceWorkFactory<C>;
	type RepeatedWorkFactory = TickedRepeatingWorkFactory<C>;
	type InvokedWorkFactory = TickedInvokedWorkFactory<C>;
	type ContinuousWorkFactory = TickedContinuousWorkFactory<C>;

	fn schedule_work<W>(&mut self, mut work: W, cancellation_id: WorkCancellationId)
	where
		W: 'static + ScheduledWork<Tick = Tick, WorkContextProvider = C> + Send + Sync,
	{
		work.on_scheduled_hook(self.current_tick);

		self.action_queue.push(ScheduledWorkAction::Activate((
			cancellation_id,
			Box::new(work),
		)));

		//	let mut s = SubscriptionData::<ContextProvider>::default();
		// TODO: Try returning subscriptions instead of ownerids
	}

	fn schedule_invoked_work<W>(&mut self, mut work: W, invoke_id: WorkInvokeId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync,
	{
		work.on_scheduled_hook(self.current_tick);

		self.action_queue
			.push(ScheduledWorkAction::AddInvoked((invoke_id, Box::new(work))));
	}

	fn invoke(&mut self, invoke_id: WorkInvokeId) {
		self.action_queue
			.push(ScheduledWorkAction::Invoke(invoke_id));
	}

	#[inline]
	fn cancel_invoked(&mut self, invoke_id: WorkInvokeId) {
		self.action_queue
			.push(ScheduledWorkAction::CancelInvoked(invoke_id));
	}

	fn cancel(&mut self, owner_id: WorkCancellationId) {
		self.action_queue
			.push(ScheduledWorkAction::Cancel(owner_id));
	}

	fn generate_cancellation_id(&mut self) -> WorkCancellationId {
		self.cancellation_id_generator.get_next()
	}

	fn generate_invoke_id(&mut self) -> WorkInvokeId {
		self.invoke_id_generator.get_next()
	}
}
