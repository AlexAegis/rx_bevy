use crate::{
	CommandSubscriber, NoopSubscription, ObservableComponent, ObservableOnInsertContext,
	SignalBound, SubscriptionComponent, observable_on_insert_hook, observable_on_remove_hook,
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
	In: SignalBound,
	InError: SignalBound,
{
	/// The entity that observes [Rx] events for this entity
	// TODO: This too could be in another component using a relationship
	subject_observer_entity: Option<Entity>,

	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Component for SubjectComponent<In, InError>
where
	In: Clone + SignalBound,
	InError: Clone + SignalBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<<Self as ObservableComponent>::Subscription>);
	}
}

impl<In, InError> SubjectComponent<In, InError>
where
	In: Clone + SignalBound,
	InError: Clone + SignalBound,
{
	pub fn new() -> Self {
		Self {
			subject_observer_entity: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableComponent for SubjectComponent<In, InError>
where
	In: Clone + SignalBound,
	InError: Clone + SignalBound,
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

	fn on_subscribe(&mut self, _subscriber: CommandSubscriber<In, InError>) -> Self::Subscription {
		NoopSubscription::default()
	}
}

/// Manually triggered events should trigger all subscribers
pub fn forward_to_subscribers<O>(
	trigger: Trigger<RxSignal<O::Out, O::OutError>>,
	mut observable_subscriptions_query: Query<&mut Subscriptions<O::Subscription>, With<O>>,
	subscription_query: Query<&SubscriptionComponent<O::Subscription>>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: Clone + SignalBound,
	O::OutError: Clone + SignalBound,
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
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for SubjectComponent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}
