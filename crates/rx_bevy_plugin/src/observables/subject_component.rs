use crate::{
	CommandSubscriber, NoopSubscription, ObservableComponent, ObservableOnInsertContext,
	ObservableSignalBound, Subscribe, SubscriptionComponent, WithSubscribeObserverReference,
	observable_on_insert_hook, observable_on_remove_hook,
};
use crate::{RxSignal, Subscriptions};

use std::fmt::Debug;
use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, Mutable, StorageType},
	entity::Entity,
	name::Name,
	observer::Trigger,
	query::With,
	system::{Commands, Query},
};

use rx_bevy_observable::{ObservableOutput, ObserverInput};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// A component that turns an entity into a multicast source, can observe
/// multiple other observables, and other entities can subscribe to it.
#[derive(Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	/// The entity that observes [Subscribe] events for this entity
	subscribe_observer_entity: Option<Entity>,
	/// The entity that observes [Rx] events for this entity
	subject_observer_entity: Option<Entity>,

	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Component for SubjectComponent<In, InError>
where
	In: Clone + ObservableSignalBound,
	InError: Clone + ObservableSignalBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<Self>);
	}
}

impl<In, InError> SubjectComponent<In, InError>
where
	In: Clone + ObservableSignalBound,
	InError: Clone + ObservableSignalBound,
{
	pub fn new() -> Self {
		Self {
			subscribe_observer_entity: None,
			subject_observer_entity: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> WithSubscribeObserverReference for SubjectComponent<In, InError>
where
	In: Clone + ObservableSignalBound,
	InError: Clone + ObservableSignalBound,
{
	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer_entity
	}

	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity> {
		self.subscribe_observer_entity
			.replace(subscribe_observer_entity)
	}
}

impl<In, InError> ObservableComponent for SubjectComponent<In, InError>
where
	In: Clone + ObservableSignalBound,
	InError: Clone + ObservableSignalBound,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscription = NoopSubscription<In, InError>;

	fn on_insert(&mut self, context: ObservableOnInsertContext) {
		let subject_observer_entity = context
			.commands
			.spawn((
				Name::new(format!(
					"Observer Subject Subscriber {}",
					context.observable_entity
				)),
				bevy_ecs::prelude::Observer::new(forward_to_subscribers::<Self>)
					.with_entity(context.observable_entity),
			))
			.id();

		self.subject_observer_entity = Some(subject_observer_entity);
	}

	fn on_subscribe(
		&mut self,
		_subscriber: CommandSubscriber<In, InError>,
		_subscribe_event: &Subscribe<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		NoopSubscription::default()
	}
}

/// Manually triggered events should trigger all subscribers
pub fn forward_to_subscribers<O>(
	trigger: Trigger<RxSignal<O::Out, O::OutError>>,
	mut observable_subscriptions_query: Query<&mut Subscriptions<O>, With<O>>,
	subscription_query: Query<&SubscriptionComponent<O>>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: Clone + ObservableSignalBound,
	O::OutError: Clone + ObservableSignalBound,
{
	let Ok(subscriptions) = observable_subscriptions_query.get_mut(trigger.target()) else {
		return;
	};

	// This could easily cause an infinite loop if not for CAN_SELF_SUBSCRIBE
	commands.trigger_targets(
		trigger.event().clone(),
		subscriptions.get_subscribers(&subscription_query),
	);
}

impl<In, InError> ObserverInput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type Out = In;
	type OutError = InError;
}
