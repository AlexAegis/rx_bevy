use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, SocketConnectorTarget};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

// this will define how events are triggered and what can be observed, so another associative type on the Signal defines what events the signal provides!!!
// TODO: This needs a way to define how writes accumulate if multiple connectors write into it, and since this is a relationship, it will be dropped as soon as it gets empty, so it has to be another entity
// TODO: Socket connecting idea, the connector component relates to another component, not a socket, to track emitters, acting as a source for sockets of this A here, even multiple. This could be used with observers to see if a connector tries to focus something, it will spawn a socket-connector-target and a system will ensure that empty socket-connector-targets are despawned for speed. And then actual converter to target socket forwarding happens over this target component and matching actionSocket component, this ensures that only targeted/connected sockets are processed, again, performance.
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
	asd: f32,
	#[reflect(ignore)]
	_phantom_data_action: PhantomData<A>,
}
