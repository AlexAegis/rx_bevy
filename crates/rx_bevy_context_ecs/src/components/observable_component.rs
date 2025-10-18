use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::{BevyError, ErrorContext},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query, StaticSystemParam},
	world::DeferredWorld,
};
use bevy_log::error;
use rx_bevy_core::Observable;
use short_type_name::short_type_name;
use thiserror::Error;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, EntitySubscriber,
	EntitySubscriptionContextAccessProvider, ErasedEntitySubscriber, Subscribe,
	SubscriptionComponent,
};

#[derive(Component)]
#[component(on_insert=observable_on_insert::<O, ContextAccess>, on_remove=observable_on_remove::<O, ContextAccess>)]
pub struct ObservableComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	observable: O,
	/// Relationship for the subscriptions spawned from this observable in
	/// order to despawn them together with the observable.
	subscriptions: Vec<Entity>,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<O, ContextAccess> ObservableComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	fn new(observable: O) -> Self {
		Self {
			observable,
			subscriptions: Vec::new(),
			_phantom_data: PhantomData,
		}
	}
}

fn subscribe_event_observer<'w, O, ContextAccess>(
	on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	mut commands: Commands,
	mut observable_query: Query<&mut ObservableComponent<O, ContextAccess>>,
	mut context: StaticSystemParam<'w, 'w, BevySubscriptionContext<ContextAccess>>,
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

	let destination =
		ErasedEntitySubscriber::<O::Out, O::OutError, ContextAccess>::new(event.destination_entity);
	let subscription = observable.observable.subscribe(destination, &mut context);

	let s = SubscriptionComponent::<O, ContextAccess>::new(event.observable_entity, subscription);

	commands.spawn(s);

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
		// TODO: Do I need an actual relationship? Check despawning behavior SubscribeObserverOf::<O>::new(observable_entity),
		ChildOf(hook_context.entity),
		Name::new(format!("Subscribe Observer {}", short_type_name::<O>())),
		Observer::new(subscribe_event_observer::<O, ContextAccess>)
			.with_entity(hook_context.entity)
			.with_error_handler(default_on_subscribe_error_handler),
	));
}

fn observable_on_remove<O, ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
}

/// Errors that can happen during a [Subscribe] event.
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
	// #[error("Tried to subscribe to {0}. But it disallows subscriptions from the same entity {1}.")]
	// SelfSubscribeDisallowed(String, Entity),
	// #[error(
	// 	"Tried to subscribe to a scheduled observable with an unscheduled Subscription! {0} {1}"
	// )]
	// UnscheduledSubscribeOnScheduledObservable(String, Entity),
	// #[error(
	// 	"Tried to subscribe to an unscheduled observable with a scheduled Subscription! {0} {1}"
	// )]
	// ScheduledSubscribeOnUnscheduledObservable(String, Entity),
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
