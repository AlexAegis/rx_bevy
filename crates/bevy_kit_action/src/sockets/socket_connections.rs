use std::marker::PhantomData;

use bevy::{ecs::observer::TriggerTargets, prelude::*};
use derive_where::derive_where;

use crate::{Action, SocketConnectorSource};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

/// Tracks what connectors write into this entity
#[derive(Component, Debug, Reflect)]
#[relationship_target(relationship = SocketConnectorSource::<A>)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
#[derive_where(Default)]
pub struct SocketConnections<A: Action> {
	#[relationship]
	targets: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> SocketConnections<A> {
	pub fn get_trigger_targets(&self) -> Vec<Entity> {
		self.targets.clone()
	}
}
