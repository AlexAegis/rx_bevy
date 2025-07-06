use crate::{
	DebugBound, ObservableComponent, ObservableOnInsertContext, ObservableOnRxEventContext,
	ObservableOnSubscribeContext, ObservableSignalBound, ScheduledSubscription,
	on_observable_insert_hook, on_observable_remove_hook,
};
use crate::{RxNext, Subscriptions};
use bevy::ecs::component::{Mutable, StorageType};
use bevy::prelude::*;
use rx_bevy::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct SubjectComponentSubscriber<In, InError>
where
	In: 'static + Send + Sync + Clone + DebugBound,
	InError: 'static + Send + Sync + Clone + DebugBound,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ObservableOutput for SubjectComponentSubscriber<In, InError>
where
	In: 'static + Send + Sync + Clone + DebugBound,
	InError: 'static + Send + Sync + Clone + DebugBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> ScheduledSubscription for SubjectComponentSubscriber<In, InError>
where
	In: 'static + Send + Sync + Clone + DebugBound,
	InError: 'static + Send + Sync + Clone + DebugBound,
{
	fn on_event(&mut self, event: RxNext<In>, context: ObservableOnRxEventContext) {
		// next in, trigger on subscriber!
		println!("subject tick!");
	}
}

/// A component that turns an entity into a multicast source, can observe
/// multiple other observables, and other entities can subscribe to it.
#[derive(Debug, Reflect)]
pub struct SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	/// The entity that observes [Subscribe] events for this entity
	subscribe_observer_entity: Option<Entity>,
	/// The entity that observes [Rx] events for this entity
	subject_observer_entity: Option<Entity>,

	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Component for SubjectComponent<In, InError>
where
	In: 'static + Clone + ObservableSignalBound,
	InError: 'static + Clone + ObservableSignalBound,
{
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(on_observable_insert_hook::<Self>);
		hooks.on_remove(on_observable_remove_hook::<Self>);
	}
}

impl<In, InError> SubjectComponent<In, InError>
where
	In: 'static + Clone + ObservableSignalBound,
	InError: 'static + Clone + ObservableSignalBound,
{
	pub fn new() -> Self {
		Self {
			subscribe_observer_entity: None,
			subject_observer_entity: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableComponent for SubjectComponent<In, InError>
where
	In: 'static + Clone + ObservableSignalBound,
	InError: 'static + Clone + ObservableSignalBound,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type ScheduledSubscription = SubjectComponentSubscriber<In, InError>;

	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer_entity
	}

	fn set_subscribe_observer_entity(&mut self, subscribe_observer_entity: Entity) {
		self.subscribe_observer_entity = Some(subscribe_observer_entity);
	}

	fn on_insert(&mut self, context: ObservableOnInsertContext) {
		let subject_observer_entity = context
			.commands
			.spawn((
				Name::new(format!(
					"Observer Subject Subscriber {}",
					context.observable_entity
				)),
				bevy_ecs::prelude::Observer::new(forward_to_subscriptions::<Self>),
			))
			.id();

		self.subject_observer_entity = Some(subject_observer_entity);
	}

	// TODO: Return value should describe how to clean up
	fn on_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_subscription_context: ObservableOnSubscribeContext,
	) -> Self::ScheduledSubscription {
		SubjectComponentSubscriber {
			_phantom_data: PhantomData,
		}
	}
}

pub fn forward_to_subscriptions<O>(
	trigger: Trigger<RxNext<O::Out>>,
	mut observable_subscriptions_query: Query<&mut Subscriptions<O>, With<O>>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: Clone + ObservableSignalBound,
	O::OutError: Clone + ObservableSignalBound,
{
	let Ok(subscriptions) = observable_subscriptions_query.get_mut(trigger.target()) else {
		return;
	};

	println!("forward to subs {:?}", subscriptions.get_subscriptions());

	// This could easily cause an infinite loop if not for CAN_SELF_SUBSCRIBE
	commands.trigger_targets(trigger.event().clone(), subscriptions.get_subscriptions());
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
