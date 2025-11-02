use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use rx_core_traits::{
	Observer as RxObserver, ObserverInput, Subscriber, SubscriptionLike, Tick, Tickable,
	WithSubscriptionContext,
};
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ConsumableSubscriberNotificationEvent, SubscriberNotificationEvent,
};

#[derive(Component)]
#[component(on_insert=subscriber_on_insert::<Destination>, on_remove=subscriber_on_remove::<Destination>)]
#[require( Name::new(format!("Subscriber ({})", short_type_name::<Destination>())))]
pub struct SubscriberComponent<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	this_entity: Entity,
	/// This isn't actually optional, it is just to let SharedDestination steal
	/// it for a moment and then put it back. And even that only happens with
	/// the RcSubscriber.
	pub(crate) destination: Option<Destination>,
}

impl<Destination> SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(subscriber: Destination, this_entity: Entity) -> Self {
		Self {
			this_entity,
			destination: Some(subscriber),
		}
	}

	fn get_destination(&self) -> &Destination {
		self.destination.as_ref().unwrap_or_else(|| {
			panic!(
				"{}'s shared destination in {} is stolen!",
				short_type_name::<Self>(),
				short_type_name::<SubscriberComponent<Destination>>()
			)
		})
	}

	fn get_destination_mut(&mut self) -> &mut Destination {
		self.destination.as_mut().unwrap_or_else(|| {
			panic!(
				"{}'s shared destination in {} is stolen!",
				short_type_name::<Self>(),
				short_type_name::<SubscriberComponent<Destination>>()
			)
		})
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

fn subscriber_notification_observer<'w, 's, Destination>(
	mut subscriber_notification: Trigger<
		ConsumableSubscriberNotificationEvent<Destination::In, Destination::InError>,
	>,
	context_param: BevySubscriptionContextParam<'w, 's>,
) -> Result<(), BevyError>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let subscriber_entity = subscriber_notification.target();
	let mut context = context_param.into_context(subscriber_entity);

	let mut stolen_destination =
		context.steal_subscriber_destination::<Destination>(subscriber_entity)?;

	let event = subscriber_notification.event_mut().consume();

	match event {
		SubscriberNotificationEvent::Next(next) => stolen_destination.next(next, &mut context),
		SubscriberNotificationEvent::Error(error) => stolen_destination.error(error, &mut context),
		SubscriberNotificationEvent::Complete => {
			stolen_destination.complete(&mut context);
		}
		SubscriberNotificationEvent::Tick(tick) => {
			stolen_destination.tick(tick, &mut context);
		}
		SubscriberNotificationEvent::Add(Some(teardown)) => {
			stolen_destination.add_teardown(teardown, &mut context);
		}
		SubscriberNotificationEvent::Add(None) => {}
		SubscriberNotificationEvent::Unsubscribe => {
			stolen_destination.unsubscribe(&mut context);
			context
				.deferred_world
				.commands()
				.entity(subscriber_entity)
				.despawn();
		}
	}

	context.return_stolen_subscriber_destination(subscriber_entity, stolen_destination)?;

	Ok(())
}

fn subscriber_on_insert<Destination>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(
		Observer::new(subscriber_notification_observer::<Destination>)
			.with_entity(hook_context.entity),
	);
}

fn subscriber_on_remove<Destination>(deferred_world: DeferredWorld, hook_context: HookContext)
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let context_param: BevySubscriptionContextParam = deferred_world.into();
	let mut context = context_param.into_context(hook_context.entity);
	// TODO: flip these api's from context to the component
	let mut stolen_destination = context
		.steal_subscriber_destination::<Destination>(hook_context.entity)
		.unwrap();
	stolen_destination.unsubscribe(&mut context);
	context
		.return_stolen_subscriber_destination(hook_context.entity, stolen_destination)
		.unwrap();
}

impl<Destination> ObserverInput for SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Destination> Tickable for SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		// Tick must not be stopped even if it's closed, in case a
		// downstream subscription is expecting it
		self.get_destination_mut().tick(tick, context);
	}
}

impl<Destination> RxObserver for SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		self.get_destination_mut().next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		self.get_destination_mut().error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		self.get_destination_mut().complete(context);
	}
}

impl<Destination> SubscriptionLike for SubscriberComponent<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.get_destination().is_closed()
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		self.get_destination_mut().unsubscribe(context);
		context
			.deferred_world
			.commands()
			.entity(self.this_entity)
			.try_despawn();
	}

	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		self.get_destination_mut().add_teardown(teardown, context);
	}
}
