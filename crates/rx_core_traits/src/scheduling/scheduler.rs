use std::time::Duration;

use derive_where::derive_where;

use crate::{
	ContextProvider, ContinuousTaskFactory, DelayedTaskFactory, ImmediateTaskFactory,
	InvokedTaskFactory, RepeatedTaskFactory, ScheduledOnceWork, ScheduledRepeatedWork,
	SchedulerHandle, Task, TaskCancellationId, TaskInvokeId, WithContextProvider,
	WithTaskInputOutput,
};

/// Schedulers define a set of tasks that can be offloaded to the scheduler to
/// be executed, and cancelled when no longer needed.
///
/// Store schedulers by a [SchedulerHandle]!
pub trait Scheduler: WithTaskInputOutput + WithContextProvider + Send + Sync {
	// Different types of tasks defined on the scheduler, so different  environments can create different ones, with just one subscriber
	// type DelayedTask: DelayedTaskFactory?;

	type DelayedTaskFactory: DelayedTaskFactory<Self::Tick, Self::ContextProvider>;
	type RepeatedTaskFactory: RepeatedTaskFactory<Self::Tick, Self::ContextProvider>;
	type ContinuousTaskFactory: ContinuousTaskFactory<Self::Tick, Self::ContextProvider>;
	type ImmediateTaskFactory: ImmediateTaskFactory<Self::Tick, Self::ContextProvider>;
	type InvokedTaskFactory: InvokedTaskFactory<Self::Tick, Self::ContextProvider>;

	fn schedule_task<T>(&mut self, task: T, cancellation_id: TaskCancellationId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync;

	fn schedule_invoked_task<T>(&mut self, task: T, invoke_id: TaskInvokeId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync;

	fn cancel(&mut self, cancellation_id: TaskCancellationId);

	fn invoke(&mut self, invoke_id: TaskInvokeId);

	fn cancel_invoked(&mut self, invoke_id: TaskInvokeId);

	fn generate_cancellation_id(&mut self) -> TaskCancellationId;

	fn generate_invoke_id(&mut self) -> TaskInvokeId;
}

#[derive_where(Debug)]
pub enum ScheduledTaskAction<TickInput, Context>
where
	Context: ContextProvider,
{
	Activate(
		#[derive_where(skip)]
		(
			TaskCancellationId,
			Box<dyn Task<Tick = TickInput, ContextProvider = Context> + Send + Sync>,
		),
	),
	Cancel(TaskCancellationId),
	AddInvoked(
		#[derive_where(skip)]
		(
			TaskInvokeId,
			Box<dyn Task<Tick = TickInput, ContextProvider = Context> + Send + Sync>,
		),
	),
	Invoke(TaskInvokeId),
	CancelInvoked(TaskInvokeId),
}

pub trait TaskExecutor: WithTaskInputOutput + WithContextProvider {
	type Scheduler: Scheduler<Tick = Self::Tick, ContextProvider = Self::ContextProvider>;

	fn get_scheduler_handle(&self) -> SchedulerHandle<Self::Scheduler>;
}

pub trait SchedulerScheduleTaskExtension: Scheduler {
	fn schedule_delayed_task<Work>(
		&mut self,
		work: Work,
		delay: Duration,
		owner_id: TaskCancellationId,
	) where
		Work: ScheduledOnceWork<Self::Tick, Self::ContextProvider>,
	{
		self.schedule_task(Self::DelayedTaskFactory::new(work, delay), owner_id)
	}

	fn schedule_repeated_task<Work>(
		&mut self,
		work: Work,
		interval: Duration,
		start_immediately: bool,
		owner_id: TaskCancellationId,
	) where
		Work: ScheduledRepeatedWork<Self::Tick, Self::ContextProvider>,
	{
		self.schedule_task(
			Self::RepeatedTaskFactory::new(work, interval, start_immediately),
			owner_id,
		)
	}

	fn schedule_continuous_task<Work>(&mut self, work: Work, owner_id: TaskCancellationId)
	where
		Work: ScheduledRepeatedWork<Self::Tick, Self::ContextProvider>,
	{
		self.schedule_task(Self::ContinuousTaskFactory::new(work), owner_id)
	}

	fn schedule_immediate_task<Work>(&mut self, work: Work, owner_id: TaskCancellationId)
	where
		Work: ScheduledOnceWork<Self::Tick, Self::ContextProvider>,
	{
		self.schedule_task(Self::ImmediateTaskFactory::new(work), owner_id)
	}
}

impl<S> SchedulerScheduleTaskExtension for S where S: Scheduler {}
