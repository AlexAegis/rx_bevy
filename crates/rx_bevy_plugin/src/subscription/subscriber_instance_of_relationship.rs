use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::TypePath;
use smallvec::SmallVec;

use crate::{RxSubscription, SignalBound};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Part of a Subscription, tracking the [ObservableComponent] or
/// [OperatorComponent] where it was spawned from.
///
#[derive(Component)]
#[relationship(relationship_target=SubscriberInstances<Sub>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(type_path = false))]
pub struct SubscriberInstanceOf<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	/// A reference to either the [ObservableComponent] or [OperatorComponent]
	/// that spawned this entity. The actual [RxSubscription] or [RxSubscriber]
	/// is stored in the [Subscription] component.
	#[relationship]
	instance_of: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<Sub>,
}

impl<Sub> SubscriberInstanceOf<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(instance_of: Entity) -> Self {
		Self {
			instance_of,
			_phantom_data: PhantomData,
		}
	}

	pub fn get_instance_of(&self) -> Entity {
		self.instance_of
	}
}

/// When this component is removed (Which also happens when the
/// [ObservableComponent] or [OperatorComponent] this belongs to, by the nature
/// of sharing their Subscription type), it will despawn the Subscription entities
/// referenced here, unsubscribing, and tearing down any subscription pipelines
/// it was part of.
#[derive(Component)]
#[relationship_target(relationship=SubscriberInstanceOf<Sub>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(type_path = false))]
pub struct SubscriberInstances<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	#[relationship]
	instances: SmallVec<[Entity; 1]>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<Sub>,
}

impl<Sub> SubscriberInstances<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn get_instances(&self) -> Vec<Entity> {
		self.instances.to_vec()
	}
}

#[cfg(feature = "reflect")]
impl<Sub> TypePath for SubscriberInstanceOf<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn crate_name() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn module_path() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn short_type_path() -> &'static str {
		"SubscriberInstanceOf"
	}

	fn type_ident() -> Option<&'static str> {
		Some("SubscriberInstanceOf")
	}
	fn type_path() -> &'static str {
		"rx_bevy_plugin::SubscriberInstanceOf"
	}
}

#[cfg(feature = "reflect")]
impl<Sub> TypePath for SubscriberInstances<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn crate_name() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn module_path() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn short_type_path() -> &'static str {
		"SubscriberInstances"
	}

	fn type_ident() -> Option<&'static str> {
		Some("SubscriberInstances")
	}
	fn type_path() -> &'static str {
		"rx_bevy_plugin::SubscriberInstances"
	}
}
