use bevy_ecs::observer::Trigger;
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
