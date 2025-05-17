use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, SocketConnectorTarget};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

/// Tracks what connectors write into this entity
#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[relationship_target(relationship = SocketConnectorTarget::<A>)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
#[derive_where(Default)]
pub struct SocketConnections<A: Action> {
	#[deref]
	#[relationship]
	sources: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data_action: PhantomData<A>,
}
