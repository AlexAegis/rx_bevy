use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::{BevyError, ErrorContext},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use bevy_log::error;
use rx_bevy_context::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ScheduledSubscriptionComponent,
};
use rx_core_traits::{Observable, SubscriptionLike};
use short_type_name::short_type_name;
use thiserror::Error;

use crate::{
	EntityObserver, ObservableSubscriptions, Subscribe, SubscribeObserverOf, SubscribeObserverRef,
	SubscriptionOf,
};

#[derive(Component)]
#[component(on_insert=observable_on_insert::<O>, on_remove=observable_on_remove::<O>)]
#[require(ObservableSubscriptions::<O>)]
pub struct ObservableComponent<O>
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	/// Stealable
	observable: Option<O>,
}

impl<O> ObservableComponent<O>
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(observable: O) -> Self {
		Self {
			observable: Some(observable),
		}
	}

	pub(crate) fn steal_observable(&mut self) -> O {
		self.observable
			.take()
			.expect("Observable was already stolen!")
	}

	pub(crate) fn return_stolen_observable(&mut self, observable: O) {
		if self.observable.replace(observable).is_some() {
			panic!("An observable was returned but it wasn't stolen from here!")
		}
	}
}

fn observable_on_insert<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<O>(&mut deferred_world);

	let _subscribe_event_observer_id = deferred_world
		.commands()
		.spawn((
			// TODO(bevy-0.17): This is actually not needed, it's only here to not let these observes occupy the top level in the worldentityinspector. reconsider to only use either this or the other relationship if it's still producing warnings on despawn in 0.17
			ChildOf(hook_context.entity),
			SubscribeObserverOf::<O>::new(hook_context.entity),
			Name::new(format!("Subscribe Observer {}", short_type_name::<O>())),
			Observer::new(subscribe_event_observer::<O>)
				.with_entity(hook_context.entity)
				.with_error_handler(default_on_subscribe_error_handler),
		))
		.id();
}

fn subscribe_event_observer<'w, 's, O>(
	on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	context_param: BevySubscriptionContextParam<'w, 's>,
) -> Result<(), BevyError>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let event = on_subscribe.event();

	let mut context = context_param.into_context(event.subscription_entity);

	let mut stolen_observable = context.steal_observable::<O>(event.observable_entity)?;

	let subscription = stolen_observable.subscribe(
		EntityObserver::<O::Out, O::OutError>::new(event.destination_entity),
		&mut context, // I have to access the context, passing it into something that was accessed from the context
	);

	context.return_stolen_observable(event.observable_entity, stolen_observable)?;

	let mut commands = context.deferred_world.commands();
	let mut subscription_entity_commands = commands.entity(event.subscription_entity);

	if !subscription.is_closed() {
		// Instead of spawning a new entity here, a pre-spawned one is used that the user
		// already has access to.
		// It also already contains the [SubscriptionSchedule] component.
		subscription_entity_commands.insert((
			ScheduledSubscriptionComponent::<O::Subscription>::new(
				subscription,
				event.subscription_entity,
			),
			SubscriptionOf::<O>::new(event.observable_entity),
		));
	} else {
		subscription_entity_commands.try_despawn();
	}

	Ok(())
}

/// Remove related components along with the observable
fn observable_on_remove<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<O>>()
		.remove::<SubscribeObserverRef<O>>();
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

pub trait BevySubscriptionContextExt {
	fn steal_observable<O>(&mut self, entity: Entity) -> Result<O, BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync;

	fn return_stolen_observable<O>(
		&mut self,
		entity: Entity,
		observable: O,
	) -> Result<(), BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync;
}

impl<'w, 's> BevySubscriptionContextExt for BevySubscriptionContext<'w, 's> {
	fn steal_observable<O>(&mut self, entity: Entity) -> Result<O, BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
	{
		let mut obserable_component =
			self.try_get_component_mut::<ObservableComponent<O>>(entity)?;
		Ok(obserable_component.steal_observable())
	}

	fn return_stolen_observable<O>(
		&mut self,
		entity: Entity,
		observable: O,
	) -> Result<(), BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
	{
		let mut obserable_component =
			self.try_get_component_mut::<ObservableComponent<O>>(entity)?;

		obserable_component.return_stolen_observable(observable);

		Ok(())
	}
}
