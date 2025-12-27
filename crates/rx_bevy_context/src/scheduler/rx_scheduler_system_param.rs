use bevy_app::Last;
use bevy_ecs::{
	schedule::ScheduleLabel,
	system::{ResMut, SystemParam},
};
use bevy_time::Virtual;
use rx_bevy_common::Clock;
use rx_core_traits::{SchedulerHandle, WorkExecutor};

use crate::{RxBevyExecutor, RxBevyScheduler};

/// Used for cleanup, like despawning entities
/// (`RxSchedule<'w, Last, Virtual>`)
pub type RxScheduleDespawn<'w> = RxSchedule<'w, Last, Virtual>;

/// SystemParam to access scheduler handles to run scheduled work
/// under certain bevy schedules like `Update` or `PostUpdate` and
/// a clock like `Virtual` or `Real`.
#[derive(SystemParam)]
pub struct RxSchedule<'w, S, C = Virtual>
where
	S: ScheduleLabel,
	C: Clock,
{
	executor: ResMut<'w, RxBevyExecutor<S, C>>,
}

impl<'w, S, C> RxSchedule<'w, S, C>
where
	S: ScheduleLabel,
	C: Clock,
{
	pub fn handle(&self) -> SchedulerHandle<RxBevyScheduler> {
		self.executor.get_scheduler_handle()
	}
}
