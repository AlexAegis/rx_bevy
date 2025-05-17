use std::marker::PhantomData;

use crate::{Action, SocketConnections};
use bevy::prelude::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

/// Optional component, when present next to a Connector with the same
/// FromAction type, will use the targeted entity to find the socket
#[derive(Component, Debug, Deref, DerefMut)]
#[relationship(relationship_target = SocketConnections<A>)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component, Debug))]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(
	all(feature = "inspector", feature = "reflect"),
	reflect(InspectorOptions)
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct SocketConnectorSource<A: Action> {
	#[deref]
	#[relationship]
	source: Entity,
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> SocketConnectorSource<A> {
	pub fn new(source: Entity) -> Self {
		Self {
			source,
			_phantom_data_action: PhantomData,
		}
	}
}
