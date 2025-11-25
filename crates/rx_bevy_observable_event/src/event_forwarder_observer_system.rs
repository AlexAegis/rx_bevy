use bevy_ecs::{event::Event, observer::Trigger};
use rx_core_traits::Subscriber;

use rx_bevy_context::{RxBevyContext, RxBevyContextItem};

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// events into it.
pub fn create_event_forwarder_observer_for_destination<Destination>(
	mut destination: Destination,
) -> impl FnMut(Trigger<Destination::In>, RxBevyContextItem<'_, '_>)
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	move |on_event: Trigger<Destination::In>, mut context: RxBevyContextItem| {
		let event = on_event.event().clone();
		destination.next(event, &mut context);
	}
}
