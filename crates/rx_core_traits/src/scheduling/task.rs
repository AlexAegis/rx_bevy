use std::{fmt::Debug, ops::AddAssign, time::Duration};

pub trait WithTaskInputOutput {
	/// Some schedulers pass inputs - such as the time passed - into the task
	/// to advance it.
	type Tick;
}

pub trait WithContextProvider {
	type ContextProvider: ContextProvider;
}

pub trait ContextProvider {
	type Item<'c>: TaskContext<'c>;
}

pub trait TaskContext<'c> {
	fn now(&self) -> Duration;
}

impl ContextProvider for () {
	type Item<'c> = ();
}

impl<'c> TaskContext<'c> for () {
	fn now(&self) -> Duration {
		Duration::from_millis(0)
	}
}

// pub struct TaskInput<'a, TickInput, ContextProvider>
// where
// 	ContextProvider: TaskContextProvider,
// {
// 	pub tick_input: TickInput,
// 	pub context: &'a mut ContextProvider::Item<'a>,
// }

pub trait Task: WithTaskInputOutput + WithContextProvider {
	fn tick(
		&mut self,
		task_input: Self::Tick,
		context: &mut <Self::ContextProvider as ContextProvider>::Item<'_>,
	) -> TickResult;

	/// The scheduler should calls this immediately when you pass the task into
	/// it, which happens before the first tick can.
	///
	/// TODO: VErify if it even makes sense or just defer to the next first tick
	/// on drain to act as initialize
	///
	/// TODO: ADD A RETURN VALUE AND RETURN IT TO THE USER, BUT ONLY MAKES SENSE WITH THE CONTEXT, BUT THAT CAN'T BE CALLED??
	fn on_scheduled_hook(&mut self, tick_input: Self::Tick);
}

// TODO: Scheduled tasks must be cancellable if their owner unsubscribes, or make sure they drop if their target is closed, and they must only hold weak refs

// ? Maybe split TickResult to OnceTaskTickResult and RepeatingTaskTickResult
#[derive(Debug)]
pub enum TickResult {
	Done,
	Pending,
}

impl AddAssign for TickResult {
	fn add_assign(&mut self, rhs: Self) {
		let change = match self {
			Self::Pending => Some(rhs),
			Self::Done => match rhs {
				Self::Pending => None,
				_ => Some(rhs),
			},
		};

		if let Some(change) = change {
			*self = change;
		}
	}
}

pub trait ScheduledOnceWork<TickInput, Context>:
	'static + FnOnce(TickInput, &mut Context::Item<'_>) + Send + Sync
where
	Context: ContextProvider,
{
}

impl<W, TickInput, Context> ScheduledOnceWork<TickInput, Context> for W
where
	Context: ContextProvider,
	W: 'static + FnOnce(TickInput, &mut Context::Item<'_>) + Send + Sync,
{
}

pub trait ScheduledRepeatedWork<TickInput, Context>:
	'static + FnMut(TickInput, &mut Context::Item<'_>) -> TickResult + Send + Sync
where
	Context: ContextProvider,
{
}

impl<W, TickInput, Context> ScheduledRepeatedWork<TickInput, Context> for W
where
	Context: ContextProvider,
	W: 'static + FnMut(TickInput, &mut Context::Item<'_>) -> TickResult + Send + Sync,
{
}

pub trait ImmediateTask<Work, TickInput, Context>: Task
where
	Work: ScheduledOnceWork<TickInput, Context>,
	Context: ContextProvider,
{
}

pub trait ImmediateTaskFactory<TickInput, Context>
where
	Context: ContextProvider,
{
	type Item<Work>: 'static + Task<Tick = TickInput, ContextProvider = Context> + Send + Sync
	where
		Work: ScheduledOnceWork<TickInput, Context>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, Context>;
}

pub trait RepeatedTaskFactory<TickInput, Context>
where
	Context: ContextProvider,
{
	type Item<Work>: 'static + Task<Tick = TickInput, ContextProvider = Context> + Send + Sync
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;
}

pub trait RepeatedTask<Work, TickInput, Context>: Task
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: ContextProvider,
{
}

pub trait ContinuousTaskFactory<TickInput, Context>
where
	Context: ContextProvider,
{
	type Item<Work>: 'static + Task<Tick = TickInput, ContextProvider = Context> + Send + Sync
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;
}

pub trait ContinuousTask<Work, TickInput, Context>: Task
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: ContextProvider,
{
}

pub trait DelayedTask<Work, TickInput, Context>: Task
where
	Work: ScheduledOnceWork<TickInput, Context>,
	Context: ContextProvider,
{
}

pub trait DelayedTaskFactory<TickInput, Context>
where
	Context: ContextProvider,
{
	type Item<Work>: 'static + Task<Tick = TickInput, ContextProvider = Context> + Send + Sync
	where
		Work: ScheduledOnceWork<TickInput, Context>;

	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, Context>;
}

pub trait InvokedTask<Work, TickInput, Context>: Task
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: ContextProvider,
{
}

pub trait InvokedTaskFactory<TickInput, Context>
where
	Context: ContextProvider,
{
	type Item<Work>: 'static + Task<Tick = TickInput, ContextProvider = Context> + Send + Sync
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;
}
