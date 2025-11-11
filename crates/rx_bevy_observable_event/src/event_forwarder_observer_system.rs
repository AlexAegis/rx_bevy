use bevy_ecs::{entity::Entity, event::Event, observer::Trigger};
use rx_core_traits::Subscriber;

use rx_bevy_context::{BevySubscriptionContextParam, BevySubscriptionContextProvider};

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// events into it.
pub fn create_event_forwarder_observer_for_destination<Destination>(
	mut destination: Destination,
	contextual_subscription_entity: Entity,
) -> impl FnMut(Trigger<Destination::In>, BevySubscriptionContextParam<'_, '_>)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	move |on_event: Trigger<Destination::In>, context_param: BevySubscriptionContextParam| {
		let mut context = context_param.into_context(contextual_subscription_entity);
		let event = on_event.event().clone();
		destination.next(event, &mut context);
	}
}
