use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_traits::{
	RepeatedTaskFactory, ScheduledRepeatedWork, Task, TaskContextProvider, Tick, TickResult,
	WithTaskInputOutput,
};

pub struct RepeatedTaskTickedFactory<ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	_phantom_data: PhantomData<fn(ContextProvider) -> ContextProvider>,
}

impl<ContextProvider> RepeatedTaskFactory<Tick, ContextProvider>
	for RepeatedTaskTickedFactory<ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider,
{
	type Item<Work>
		= AcquireReleaseTaskTicked<Work, ContextProvider>
	where
		Work: ScheduledRepeatedWork<Tick, ContextProvider>;

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, ContextProvider>,
	{
		AcquireReleaseTaskTicked {
			start_immediately,
			consumed_until: Tick::default(),
			current_tick: Tick::default(),
			interval,
			work,
			_phantom_data: PhantomData,
		}
	}
}

/// Lets you pair an owner_id to a function call. The points would be to make sure an unsubscribe's scheduler cancellation, will run after the task had run.
/// it should maintain a stack of events that can  be consumed based on its predicates, to ensure that its two function calls, actuially onlt run in a specific order.
/// to guarantee that fun1 always comes before fun2, even if fun2 was called first.
/// it returns a releaseId for the task to uniquely identify this task, and interact with it.
///
/// this is essentialyl a WATCHER_TASK it watches a resource, and binds its lifecycle to the subscriptions
///
/// observing a TASKMODIFICATIONREQUEST in the separate schedule queue, that is run before tasks, in each drain loop
/// the taskmodificationRequest targeting task_id has a tasktype, so some dyn upcasting might be needed?
/// a close request either releases a resource, or closes the task itself, so if the open request
/// comes later, it won't try to open it.
#[derive_where(Debug)]
pub struct AcquireReleaseTaskTicked<Work, ContextProvider>
where
	Work: ScheduledRepeatedWork<Tick, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	/// The work will be executed on the first tick too, regardless if the timer
	/// had elapsed or not.
	start_immediately: bool,
	consumed_until: Tick,
	current_tick: Tick,
	interval: Duration,
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(ContextProvider) -> ContextProvider>,
}

impl<Work, ContextProvider> WithTaskInputOutput for AcquireReleaseTaskTicked<Work, ContextProvider>
where
	Work: ScheduledRepeatedWork<Tick, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
}

impl<Work, ContextProvider> Task for AcquireReleaseTaskTicked<Work, ContextProvider>
where
	Work: ScheduledRepeatedWork<Tick, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	fn tick(
		&mut self,
		tick_input: Self::TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult {
		self.current_tick.update(tick_input);

		let mut tick_result = TickResult::Pending;
		while self.consumed_until + self.interval <= self.current_tick {
			self.consumed_until += self.interval;
			tick_result += (self.work)(tick_input, context);
		}
		tick_result
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::TickInput) {
		if self.start_immediately {
			self.consumed_until.update(tick_input - self.interval);
		} else {
			self.consumed_until.update(tick_input);
		}

		self.current_tick.update(tick_input);
	}
}
