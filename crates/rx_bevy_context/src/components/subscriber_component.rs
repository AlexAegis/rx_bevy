use bevy_ecs::{
	component::{Component, HookContext},
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Query, StaticSystemParam},
	world::DeferredWorld,
};
use rx_core_traits::Subscriber;
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	ConsumableSubscriberNotificationEvent, SubscriberNotificationEvent,
	SubscriberNotificationEventError, SubscriptionNotificationEvent,
};

#[derive(Component)]
#[component(on_insert=subscriber_on_insert::<Destination>, on_remove=subscriber_on_remove::<Destination>)]
#[require(Name::new(format!("Subscriber ({})", short_type_name::<Destination>())))]
pub struct SubscriberComponent<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub(crate) destination: Destination,
}

impl<Destination> SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	pub(crate) fn new(subscriber: Destination) -> Self {
		Self {
			destination: subscriber,
		}
	}
}

fn subscriber_notification_observer<Destination>(
	mut subscriber_notification: Trigger<
		ConsumableSubscriberNotificationEvent<Destination::In, Destination::InError>,
	>,
	mut subscriber_query: Query<&mut SubscriberComponent<Destination>>,
	mut context: StaticSystemParam<BevySubscriptionContext>,
) -> Result<(), BevyError>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let subscriber_entity = subscriber_notification.target();
	let Ok(mut subscriber_component) = subscriber_query.get_mut(subscriber_entity) else {
		return Err(SubscriberNotificationEventError::NotASubscriber(
			short_type_name::<Destination>(),
			subscriber_entity,
		)
		.into());
	};

	let event = subscriber_notification
		.event_mut()
		.take()
		.expect("notification was already consumed!");

	match event {
		SubscriberNotificationEvent::Next(next) => {
			subscriber_component.destination.next(next, &mut context)
		}
		SubscriberNotificationEvent::Error(error) => {
			subscriber_component.destination.error(error, &mut context)
		}
		SubscriberNotificationEvent::Complete => {
			subscriber_component.destination.complete(&mut context);
		}
		SubscriberNotificationEvent::Tick(tick) => {
			subscriber_component.destination.tick(tick, &mut context);
		}
		SubscriberNotificationEvent::Add(Some(teardown)) => {
			subscriber_component
				.destination
				.add_teardown(teardown, &mut context);
		}
		SubscriberNotificationEvent::Unsubscribe => {
			subscriber_component.destination.unsubscribe(&mut context);
		}
		_ => {}
	}

	Ok(())
}

fn subscriber_on_insert<Destination>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		subscriber_notification_observer::<Destination>,
	));
}

fn subscriber_on_remove<Destination>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	deferred_world.commands().trigger_targets(
		SubscriptionNotificationEvent::Unsubscribe,
		hook_context.entity,
	);
}
