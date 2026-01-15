use std::marker::PhantomData;

use bevy_ecs::{resource::Resource, schedule::ScheduleLabel};
use bevy_time::Virtual;
use rx_core_common::PhantomInvariant;
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_scheduler_ticking::{Tick, TickingSchedulerExecutor};

use crate::{Clock, RxBevyContext, RxBevyContextItem, RxBevyScheduler};

// TODO: SystemParam that is the scheduler directly, maybe use the builder pattern of sysparams
#[derive(Resource, RxExecutor)]
#[rx_context(RxBevyContext)]
#[rx_tick(Tick)]
#[rx_scheduler(RxBevyScheduler)]
pub struct RxBevyExecutor<S, C = Virtual>
where
	S: ScheduleLabel,
	C: Clock,
{
	#[scheduler_handle]
	ticking_executor: TickingSchedulerExecutor<RxBevyScheduler, RxBevyContext>,
	_phantom_data: PhantomInvariant<(S, C)>,
}

impl<S, C> RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.ticking_executor.is_empty()
	}
}

impl<S, C> Default for RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	fn default() -> Self {
		Self {
			ticking_executor: TickingSchedulerExecutor::new(RxBevyScheduler::default()),
			_phantom_data: PhantomData,
		}
	}
}

impl<S, C> RxBevyExecutor<S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	pub fn tick_to<'a>(&mut self, tick: Tick, context: &mut RxBevyContextItem<'a>) {
		self.ticking_executor.tick_to(tick, context);
	}
}
