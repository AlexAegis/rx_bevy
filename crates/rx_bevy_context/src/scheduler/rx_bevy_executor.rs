use std::marker::PhantomData;

use crate::{BevyRxScheduler, RxBevyContext, RxBevyContextItem};
use bevy_ecs::{resource::Resource, schedule::ScheduleLabel, world::FromWorld};
use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::TickingSchedulerExecutor;
use rx_core_traits::{TaskExecutor, Tick, WithTaskInputOutput};

// TODO: SystemParam that is the scheduler directly, maybe use the builder pattern of sysparams
#[derive(Resource)]
pub struct RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	ticking_executor: TickingSchedulerExecutor<BevyRxScheduler<S, C>, (), RxBevyContext>,
	_phantom_data: PhantomData<(S, C)>,
}

impl<S, C> RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	pub fn tick<'a>(&mut self, tick: Tick, context: &mut RxBevyContextItem<'a, 'a>) {
		self.ticking_executor.tick(tick, context);
	}
}
impl<S, C> FromWorld for RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	fn from_world(_world: &mut bevy_ecs::world::World) -> Self {
		Self {
			ticking_executor: TickingSchedulerExecutor::new(BevyRxScheduler::new()),
			_phantom_data: PhantomData,
		}
	}
}

impl<S, C> WithTaskInputOutput for RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	type TickInput = Tick;
	type TaskError = ();
	type ContextProvider = RxBevyContext;
}

impl<S, C> TaskExecutor for RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	type Scheduler = BevyRxScheduler<S, C>;

	fn get_scheduler(&self) -> rx_core_traits::SchedulerHandle<Self::Scheduler> {
		self.ticking_executor.get_scheduler()
	}
}
