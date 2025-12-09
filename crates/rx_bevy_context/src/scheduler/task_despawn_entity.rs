use bevy_ecs::entity::Entity;
use rx_core_macro_task_derive::RxTask;
use rx_core_scheduler_ticking::Tick;
use rx_core_traits::{Scheduler, Task, TaskCancellationId, TaskInvokeId, TickResult};

use crate::{RxBevyContext, RxBevyScheduler};

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(RxBevyContext)]
pub struct SchedulerTaskDespawnEntity {
	entity: Entity,
}

impl SchedulerTaskDespawnEntity {
	pub fn new(entity: Entity) -> Self {
		Self { entity }
	}
}

impl Task for SchedulerTaskDespawnEntity {
	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}

	fn tick(
		&mut self,
		_task_input: Self::Tick,
		context: &mut <Self::ContextProvider as rx_core_traits::ContextProvider>::Item<'_>,
	) -> rx_core_traits::TickResult {
		context
			.deferred_world
			.commands()
			.entity(self.entity)
			.try_despawn();
		TickResult::Done
	}
}

pub trait RxBevySchedulerDespawnEntityExtension:
	Scheduler<Tick = Tick, ContextProvider = RxBevyContext>
{
	fn schedule_despawn_entity(&mut self, entity: Entity, owner_id: Option<TaskCancellationId>) {
		let owner_id = owner_id.unwrap_or_else(|| self.generate_cancellation_id());
		self.schedule_task(SchedulerTaskDespawnEntity::new(entity), owner_id);
	}

	fn schedule_invoked_despawn_entity(&mut self, entity: Entity, invoke_id: TaskInvokeId) {
		self.schedule_invoked_task(SchedulerTaskDespawnEntity::new(entity), invoke_id);
	}
}

impl RxBevySchedulerDespawnEntityExtension for RxBevyScheduler {}
