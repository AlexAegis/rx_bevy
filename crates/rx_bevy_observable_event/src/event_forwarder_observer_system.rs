use bevy_ecs::{event::EntityEvent, observer::On};
use rx_core_common::Subscriber;

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// events into it.
pub fn create_event_forwarder_observer_for_destination<Destination>(
	mut destination: Destination,
) -> impl FnMut(On<Destination::In>)
where
	Destination: 'static + Subscriber,
	Destination::In: EntityEvent + Clone,
{
	move |on_event: On<Destination::In>| {
		let event = on_event.event().clone();
		destination.next(event);
	}
}
