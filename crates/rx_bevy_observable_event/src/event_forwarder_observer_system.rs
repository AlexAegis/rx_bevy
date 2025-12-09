use bevy_ecs::{event::Event, observer::Trigger};
use rx_core_traits::Subscriber;

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// events into it.
pub fn create_event_forwarder_observer_for_destination<Destination>(
	mut destination: Destination,
) -> impl FnMut(Trigger<Destination::In>)
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	move |on_event: Trigger<Destination::In>| {
		let event = on_event.event().clone();
		destination.next(event);
	}
}
