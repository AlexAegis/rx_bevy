use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::{BevyError, ErrorContext},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query},
	world::DeferredWorld,
};
use bevy_log::error;
use rx_bevy_core::Observable;
use short_type_name::short_type_name;
use thiserror::Error;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	EntitySubscriptionContextAccessProvider, ErasedEntitySubscriber, ObservableSubscriptions,
	Subscribe, SubscribeObserverOf, SubscribeObserverRef, SubscriptionComponent, SubscriptionOf,
};

#[derive(Component)]
#[component(on_insert=observable_on_insert::<O, ContextAccess>, on_remove=observable_on_remove::<O, ContextAccess>)]
#[require(ObservableSubscriptions::<O, ContextAccess>)]
pub struct ObservableComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	observable: O,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<O, ContextAccess> ObservableComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	pub fn new(observable: O) -> Self {
		Self {
			observable,
			_phantom_data: PhantomData,
		}
	}
}

fn subscribe_event_observer<'w, 's, O, ContextAccess>(
	on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	mut commands: Commands,
	mut observable_query: Query<&mut ObservableComponent<O, ContextAccess>>,
	mut context: BevySubscriptionContext<'w, 's, ContextAccess>,
) -> Result<(), BevyError>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	let event = on_subscribe.event();

	let Ok(mut observable) = observable_query.get_mut(event.observable_entity) else {
		return Err(SubscribeError::NotAnObservable(
			short_type_name::<O>(),
			event.observable_entity,
		)
		.into());
	};

	let subscription = observable.observable.subscribe(
		ErasedEntitySubscriber::<O::Out, O::OutError, ContextAccess>::new(event.destination_entity),
		&mut context,
	);

	// Instead of spawning a new entity here, a pre-spawned one is used that the user
	// already has access to.
	// It also already contains the [SubscriptionSchedule] component.
	let mut subscription_entity_commands = commands.entity(event.subscription_entity);
	subscription_entity_commands.insert((
		SubscriptionComponent::<O, ContextAccess>::new(subscription),
		SubscriptionOf::<O, ContextAccess>::new(event.observable_entity),
	));

	Ok(())
}

fn observable_on_insert<O, ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	deferred_world.commands().spawn((
		// TODO(bevy-0.17): This is actually not needed, it's only here to not let these observes occupy the top level in the worldentityinspector. reconsider to only use either this or the other relationship if it's still producing warnings on despawn in 0.17
		ChildOf(hook_context.entity),
		SubscribeObserverOf::<O, ContextAccess>::new(hook_context.entity),
		Name::new(format!("Subscribe Observer {}", short_type_name::<O>())),
		Observer::new(subscribe_event_observer::<O, ContextAccess>)
			.with_entity(hook_context.entity)
			.with_error_handler(default_on_subscribe_error_handler),
	));
}

/// Remove related components along with the observable
fn observable_on_remove<O, ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<O, ContextAccess>>()
		.remove::<SubscribeObserverRef<O, ContextAccess>>();
}

/// Errors that can happen during a [Subscribe] event.
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
	// TODO: consider how this could be implemented now, or if it's even needed. self subscriptions on subjects would cause infinite loops, maybe subjects could be treated as special things and have their own components which could be used to to trigger this error with an associuated const
	// #[error("Tried to subscribe to {0}. But it disallows subscriptions from the same entity {1}.")]
	// SelfSubscribeDisallowed(String, Entity),
}

/// The default error handler just prints out the error as warning
pub(crate) fn default_on_subscribe_error_handler(error: BevyError, error_context: ErrorContext) {
	if let Some(subscribe_error) = error.downcast_ref::<SubscribeError>() {
		error!("{}", subscribe_error);
	} else {
		panic!(
			"Unknown error happened during subscribe. Kind: {}\tName: {}",
			error_context.kind(),
			error_context.name()
		);
	}
}
