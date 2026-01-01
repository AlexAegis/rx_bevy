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
	/// Schedules a task that will execute once **at least** a `delay` worth of
	/// time had passed and the executor ticks.
	///
	/// This means that a nested series of delayed work that issues more
	/// delayed work will **always** drift, and the total execution
	/// time **will be** larger than the sum of the delays. If this is a problem,
	/// use a repeated work that has its own internal timer.
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

	/// Schedules a task that will execute every time the `interval` worth of
	/// time had passed, and the executor ticks.
	///
	/// If a single tick rolls the interval over multiple times, the work will
	/// also be ticked multiple times, up to `max_work_per_tick`, but at least
	/// once.
	fn schedule_repeated_work<Work>(
		&mut self,
		work: Work,
		interval: Duration,
		start_immediately: bool,
		max_work_per_tick: usize,
		cancellation_id: WorkCancellationId,
	) where
		Work: ScheduledRepeatedWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(
			Self::RepeatedWorkFactory::new(
				work,
				interval,
				start_immediately,
				NonZero::new(max_work_per_tick).unwrap_or(NonZero::<usize>::MIN),
			),
			cancellation_id,
		)
	}

	/// Schedules a task that will execute every time the executor ticks!
	/// This can mean different things dependning on the executor and the
	/// environment it's used in.
	///
	/// For example, in a game engine, the executor is expected to be ticked
	/// once every frame, therefore a continuous work is expected to be
	/// executed once per frame. But in another environment this could mean
	/// once every 1ms. It should be ticked very often.
	fn schedule_continuous_work<Work>(&mut self, work: Work, cancellation_id: WorkCancellationId)
	where
		Work: ScheduledRepeatedWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(Self::ContinuousWorkFactory::new(work), cancellation_id)
	}

	/// Schedules a task that will execute as soon as the executor ticks!
	fn schedule_immediate_work<Work>(&mut self, work: Work, cancellation_id: WorkCancellationId)
	where
		Work: ScheduledOnceWork<Self::Tick, Self::WorkContextProvider>,
	{
		self.schedule_work(Self::ImmediateWorkFactory::new(work), cancellation_id)
	}
}

impl<S> SchedulerScheduleWorkExtension for S where S: Scheduler {}
