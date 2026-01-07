use bevy_ecs::entity::Entity;
use rx_core_macro_work_derive::RxWork;
use rx_core_scheduler_ticking::Tick;
use rx_core_traits::{ScheduledWork, Scheduler, WorkCancellationId, WorkInvokeId, WorkResult};

use crate::{RxBevyContext, RxBevyScheduler};

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(RxBevyContext)]
pub struct ScheduledWorkDespawnEntity {
	entity: Entity,
}

impl ScheduledWorkDespawnEntity {
	pub fn new(entity: Entity) -> Self {
		Self { entity }
	}
}

impl ScheduledWork for ScheduledWorkDespawnEntity {
	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}

	fn tick(
		&mut self,
		_input: Self::Tick,
		context: &mut <Self::WorkContextProvider as rx_core_traits::WorkContextProvider>::Item<'_>,
	) -> rx_core_traits::WorkResult {
		context
			.deferred_world
			.commands()
			.entity(self.entity)
			.try_despawn();
		WorkResult::Done
	}
}

pub trait RxBevySchedulerDespawnEntityExtension:
	Scheduler<Tick = Tick, WorkContextProvider = RxBevyContext>
{
	fn schedule_despawn_entity(&mut self, entity: Entity, owner_id: Option<WorkCancellationId>) {
		let owner_id = owner_id.unwrap_or_else(|| self.generate_cancellation_id());
		self.schedule_work(ScheduledWorkDespawnEntity::new(entity), owner_id);
	}

	fn schedule_invoked_despawn_entity(&mut self, entity: Entity, invoke_id: WorkInvokeId) {
		self.schedule_invoked_work(ScheduledWorkDespawnEntity::new(entity), invoke_id);
	}
}

impl RxBevySchedulerDespawnEntityExtension for RxBevyScheduler {}
