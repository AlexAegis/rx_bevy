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
	pub(crate) destination: Option<Destination>,
}

impl<Destination> SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub(crate) fn new(subscriber: Destination) -> Self {
		Self {
			destination: Some(subscriber),
		}
	}

	/// Takes the destination out of the component, and puts trust in the
	/// callers self counciousness to be returned later.
	pub(crate) fn steal_destination(&mut self) -> Destination {
		self.destination.take().unwrap_or_else(|| {
			panic!(
				"{}'s shared destination in {} was already stolen!",
				short_type_name::<Self>(),
				short_type_name::<SubscriberComponent<Destination>>()
			)
		})
	}

	pub(crate) fn return_stolen_destination(&mut self, destination: Destination) {
		let _old_destination = self.destination.replace(destination);

		#[cfg(feature = "debug")]
		if _old_destination.is_some() {
			panic!(
				"A stolen destination was returned to {} but it does not belong here!",
				short_type_name::<Self>(),
			);
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

	let destination = subscriber_component.destination.as_mut().expect(
		"destination should only be None during a shared subscripstions access through one",
	);
	match event {
		SubscriberNotificationEvent::Next(next) => destination.next(next, &mut context),
		SubscriberNotificationEvent::Error(error) => destination.error(error, &mut context),
		SubscriberNotificationEvent::Complete => {
			destination.complete(&mut context);
		}
		SubscriberNotificationEvent::Tick(tick) => {
			destination.tick(tick, &mut context);
		}
		SubscriberNotificationEvent::Add(Some(teardown)) => {
			destination.add_teardown(teardown, &mut context);
		}
		SubscriberNotificationEvent::Unsubscribe => {
			destination.unsubscribe(&mut context);
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
