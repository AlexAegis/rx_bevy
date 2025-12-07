use std::marker::PhantomData;

use bevy_ecs::schedule::ScheduleLabel;
use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::{
	DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory, RepeatedTaskTickedFactory,
	TickingExecutorsScheduler, TickingScheduler,
};
use rx_core_traits::{Scheduler, Tick, WithTaskInputOutput};

use crate::RxBevyContext;

pub struct BevyRxScheduler<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	ticking_scheduler: TickingScheduler<(), RxBevyContext>,
	_phantom_data: PhantomData<(S, C)>,
}

impl<S, C> BevyRxScheduler<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	pub(crate) fn new() -> Self {
		Self {
			ticking_scheduler: TickingScheduler::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<S, C> TickingExecutorsScheduler for BevyRxScheduler<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	#[inline]
	fn drain_queue(
		&mut self,
	) -> std::vec::Drain<'_, rx_core_traits::ScheduledTaskAction<Tick, (), RxBevyContext>> {
		self.ticking_scheduler.drain_queue()
	}

	#[inline]
	fn update_tick(&mut self, tick: Tick) {
		self.ticking_scheduler.update_tick(tick);
	}
}

impl<S, C> WithTaskInputOutput for BevyRxScheduler<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	type TickInput = Tick;
	type TaskError = ();
	type ContextProvider = RxBevyContext;
}

impl<S, C> Scheduler for BevyRxScheduler<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<(), RxBevyContext>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<(), RxBevyContext>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<(), RxBevyContext>;

	#[inline]
	fn schedule<T>(&mut self, task: T, owner_id: rx_core_traits::TaskOwnerId)
	where
		T: 'static
			+ rx_core_traits::Task<
				TickInput = Self::TickInput,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>
			+ Send
			+ Sync,
	{
		self.ticking_scheduler.schedule(task, owner_id);
	}

	#[inline]
	fn cancel(&mut self, owner_id: rx_core_traits::TaskOwnerId) {
		self.ticking_scheduler.cancel(owner_id);
	}

	#[inline]
	fn generate_owner_id(&mut self) -> rx_core_traits::TaskOwnerId {
		self.ticking_scheduler.generate_owner_id()
	}
}
