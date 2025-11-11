use bevy_ecs::{entity::Entity, observer::Trigger};
use rx_core_traits::{Subscriber, SubscriberPushNotificationExtention};

use crate::{BevySubscriptionContextParam, BevySubscriptionContextProvider, RxSignal};

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// notifications into it.
pub fn create_notification_forwarder_observer_for_destination<Destination>(
	mut destination: Destination,
	contextual_subscription_entity: Entity,
) -> impl FnMut(
	Trigger<'_, RxSignal<Destination::In, Destination::InError>>,
	BevySubscriptionContextParam<'_, '_>,
)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	move |on_event: Trigger<RxSignal<Destination::In, Destination::InError>>,
	      context_param: BevySubscriptionContextParam| {
		let mut context = context_param.into_context(contextual_subscription_entity);

		destination.push(on_event.event().clone(), &mut context);
	}
}
