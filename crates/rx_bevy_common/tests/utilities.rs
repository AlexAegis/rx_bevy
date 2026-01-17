use bevy_ecs::{entity::Entity, observer::Trigger, world::World};
use rx_bevy::RxSignal;
use rx_core_common::{Signal, SubscriberNotification};
use rx_core_testing::NotificationCollector;

/// #[allow(dead_code)]: Imported using module paths:
///
/// ```
/// #[path = "./utilities.rs"]
/// mod utilities;
/// use utilities::*;
/// ```
#[allow(dead_code)]
pub(crate) fn collect_notifications_into<In, InError>(
	notifications: NotificationCollector<In, InError>,
) -> impl FnMut(Trigger<RxSignal<In, InError>>)
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	move |trigger: Trigger<RxSignal<In, InError>>| {
		notifications
			.lock()
			.push(SubscriberNotification::from(trigger.event().clone()));
	}
}

#[allow(dead_code)]
pub(crate) fn component_count(world: &World, entity: Entity) -> usize {
	world.entity(entity).archetype().components().count()
}

#[allow(dead_code)]
pub(crate) fn component_names(world: &World, entity: Entity) -> Vec<String> {
	world
		.entity(entity)
		.archetype()
		.components()
		.filter_map(|component_id| {
			world
				.components()
				.get_info(component_id)
				.map(|info| info.name().to_string())
		})
		.collect()
}
