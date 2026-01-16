use bevy_derive::Deref;
use bevy_ecs::{
	bundle::Bundle,
	component::{Component, Mutable},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query},
};
use disqualified::ShortName;
use rx_core_common::{
	Observable, PhantomInvariant, SubscriptionLike, SubscriptionLikeExtensionIntoShared,
};

use core::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{
	ErasedSubscribeObserverOf, RxScheduleDespawn, Subscribe, SubscribeError, SubscriptionComponent,
	SubscriptionOf, UnfinishedSubscription,
};

#[derive(Bundle)]
pub struct SubscribeEventObserverSatelliteBundle<O>
where
	O: 'static + Observable + Send + Sync,
{
	name: Name,
	relationship: SubscribeObserverOf<O>,
	erased_relationship: ErasedSubscribeObserverOf<O::Out, O::OutError>,
	subscribe_event_observer: Observer,
}

impl<O> SubscribeEventObserverSatelliteBundle<O>
where
	O: 'static + Observable + Send + Sync,
{
	pub fn new<C>(entity: Entity) -> Self
	where
		C: Component<Mutability = Mutable> + Observable<Out = O::Out, OutError = O::OutError>,
	{
		Self {
			name: Name::new(format!(
				"Subscribe Observer <Out = {}, OutError = {}> ({})",
				ShortName::of::<O::Out>(),
				ShortName::of::<O::OutError>(),
				ShortName::of::<O>()
			)),
			relationship: SubscribeObserverOf::<O>::new(entity),
			erased_relationship: ErasedSubscribeObserverOf::<O::Out, O::OutError>::new(entity),
			subscribe_event_observer: Observer::new(observable_subscribe_event_observer::<C>)
				.with_entity(entity)
				.with_error_handler(bevy_ecs::error::default_error_handler()),
		}
	}
}

pub(crate) fn observable_subscribe_event_observer<O>(
	mut on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	mut commands: Commands,
	mut observable_query: Query<&mut O>,
	rx_schedule_despawn: RxScheduleDespawn,
) -> Result<(), BevyError>
where
	O: 'static + Observable + Component<Mutability = Mutable> + Send + Sync,
{
	let event = on_subscribe.event_mut();

	let Some(destination) = event.try_consume_destination() else {
		return Err(SubscribeError::EventAlreadyConsumed(
			ShortName::of::<O>().to_string(),
			event.observable_entity,
		)
		.into());
	};

	let Ok(mut observable_component) = observable_query.get_mut(event.observable_entity) else {
		return Err(SubscribeError::NotAnObservable(
			ShortName::of::<O>().to_string(),
			event.observable_entity,
		)
		.into());
	};

	let subscription = observable_component.subscribe(destination);

	let mut subscription_entity_commands = commands.entity(event.subscription_entity);

	if !subscription.is_closed() {
		// Instead of spawning a new entity here, a pre-spawned one is used that the user
		// already has access to.
		// It also already contains the [SubscriptionSchedule] component.

		subscription_entity_commands.insert((
			SubscriptionComponent::new_despawn_on_unsubscribe(
				subscription.into_shared(),
				event.subscription_entity,
				rx_schedule_despawn.handle(),
			),
			SubscriptionOf::<O>::new(event.observable_entity),
		));
	} else {
		// The subscription is already closed, despawn the pre-spawned subscription entity
		subscription_entity_commands.try_despawn();
	}

	// Marks the subscription entity as "finished".
	// An "unfinished" subscription entity would be immediately despawned.
	subscription_entity_commands.try_remove::<UnfinishedSubscription>();

	Ok(())
}

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, Debug)]
#[relationship_target(relationship=SubscribeObserverOf::<O>)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscribeObserverRef<O>
where
	O: 'static + Observable + Send + Sync,
{
	#[relationship]
	#[deref]
	subscribe_observer_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

#[derive(Component, Deref)]
#[relationship(relationship_target=SubscribeObserverRef::<O>)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscribeObserverOf<O>
where
	O: 'static + Observable + Send + Sync,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

impl<O> SubscribeObserverOf<O>
where
	O: 'static + Observable + Send + Sync,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
