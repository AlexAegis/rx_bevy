use core::{num::NonZero, time::Duration};

use derive_where::derive_where;

use crate::{
	ContinuousTaskFactory, DelayedWorkFactory, ImmediateTaskFactory, InvokedTaskFactory,
	RepeatedTaskFactory, ScheduledOnceWork, ScheduledRepeatedWork, ScheduledWork,
	WithWorkContextProvider, WithWorkInputOutput, WorkCancellationId, WorkContextProvider,
	WorkInvokeId,
};

/// Schedulers define a set of work that can be sent to the scheduler to
/// then be executed by it's executor, and cancelled when no longer needed.
pub trait Scheduler: WithWorkInputOutput + WithWorkContextProvider + Send + Sync {
	type DelayedWorkFactory: DelayedWorkFactory<Self::Tick, Self::WorkContextProvider>;
	type RepeatedWorkFactory: RepeatedTaskFactory<Self::Tick, Self::WorkContextProvider>;
	type ContinuousWorkFactory: ContinuousTaskFactory<Self::Tick, Self::WorkContextProvider>;
	type ImmediateWorkFactory: ImmediateTaskFactory<Self::Tick, Self::WorkContextProvider>;
	type InvokedWorkFactory: InvokedTaskFactory<Self::Tick, Self::WorkContextProvider>;

	fn schedule_work<W>(&mut self, work: W, cancellation_id: WorkCancellationId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync;

	fn schedule_invoked_work<W>(&mut self, work: W, invoke_id: WorkInvokeId)
	where
		W: 'static
			+ ScheduledWork<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>
			+ Send
			+ Sync;

	fn cancel(&mut self, cancellation_id: WorkCancellationId);

	fn invoke(&mut self, invoke_id: WorkInvokeId);

	fn cancel_invoked(&mut self, invoke_id: WorkInvokeId);

	fn generate_cancellation_id(&mut self) -> WorkCancellationId;

	fn generate_invoke_id(&mut self) -> WorkInvokeId;
}

#[derive_where(Debug)]
pub enum ScheduledWorkAction<TickInput, Context>
where
	Context: WorkContextProvider,
{
	Activate(
		#[derive_where(skip)]
		(
			WorkCancellationId,
			Box<dyn ScheduledWork<Tick = TickInput, WorkContextProvider = Context> + Send + Sync>,
		),
	),
	Cancel(WorkCancellationId),
	AddInvoked(
		#[derive_where(skip)]
		(
			WorkInvokeId,
			Box<dyn ScheduledWork<Tick = TickInput, WorkContextProvider = Context> + Send + Sync>,
		),
	),
	Invoke(WorkInvokeId),
	CancelInvoked(WorkInvokeId),
}

pub trait SchedulerScheduleWorkExtension: Scheduler {
	fn schedule_delayed_work<Work>(
		&mut self,
		work: Work,
		delay: Duration,
		cancellation_id: WorkCancellationId,
	) where
		Work: ScheduledOnceWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(Self::DelayedWorkFactory::new(work, delay), cancellation_id)
	}

	fn schedule_repeated_work<Work>(
		&mut self,
		work: Work,
		interval: Duration,
		start_immediately: bool,
		max_work_per_tick: NonZero<usize>,
		cancellation_id: WorkCancellationId,
	) where
		Work: ScheduledRepeatedWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(
			Self::RepeatedWorkFactory::new(work, interval, start_immediately, max_work_per_tick),
			cancellation_id,
		)
	}

	fn schedule_continuous_work<Work>(&mut self, work: Work, cancellation_id: WorkCancellationId)
	where
		Work: ScheduledRepeatedWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(Self::ContinuousWorkFactory::new(work), cancellation_id)
	}

	fn schedule_immediate_work<Work>(&mut self, work: Work, cancellation_id: WorkCancellationId)
	where
		Work: ScheduledOnceWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(Self::ImmediateWorkFactory::new(work), cancellation_id)
	}
}

impl<S> SchedulerScheduleWorkExtension for S where S: Scheduler {}
