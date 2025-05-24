use std::{any::Any, marker::PhantomData};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_egui::egui::util::id_type_map::TypeId;
use derive_where::derive_where;

use crate::{Action, ActionKeyPair, Signal, SignalTransformer, SocketConnections};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

use super::ConnectorTerminal;
/*
/// Optional component, when present next to a Connector with the same
/// ToAction type, will use the targeted entity to find the socket
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[relationship(relationship_target = SocketConnections<A>)]
pub struct SocketConnectorTarget<A: Action> {
	#[deref]
	#[relationship]
	target: Entity,
	_phantom_data_action: PhantomData<A>,
}*/
/*
impl<A: Action> SocketConnectorTarget<A> {
	pub fn new(target: Entity) -> Self {
		Self {
			target,
			_phantom_data_action: PhantomData,
		}
	}
}
*/
#[derive(Component, Debug, Reflect)]
#[require(ConnectorTerminal<ToAction>)]
#[derive_where(Default)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
pub struct SocketConnector<FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
{
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	pub default_transformer_constructor: Option<fn() -> Transformer>,
	pub(crate) signal_transformer_state: HashMap<ToAction, Transformer>,
	pub action_map: HashMap<FromAction, ToAction>,
}

impl<FromAction, ToAction, Transformer> SocketConnector<FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
{
	#[must_use]
	pub fn new(default_transformer: fn() -> Transformer) -> Self {
		Self {
			default_transformer_constructor: Some(default_transformer),
			..Default::default()
		}
	}
}

#[derive(Component, Debug, Reflect, Deref, DerefMut)]
#[require(ErasedTransformerState, TransformerOutputCache<ToAction::Signal>)]
#[derive_where(Default)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
pub struct SocketActionMap<FromAction, ToAction>
where
	FromAction: Action,
	ToAction: Action,
{
	#[deref]
	pub action_map: HashMap<FromAction, ToAction>,
}

/// Q: Why is this erased?
/// A: This is erased to ensure only one transformer is in place for
///    an [ActionKeyPair] per entity
/// Q: Why are transformers mapped over a Type and not an Instance?
/// A: Because it doesn't need to, an Action is like a channel, transformation
///    happens over an entire channel in only one way. Mapping happens within
///    a channel, between "fibers"
#[derive(Component, Debug, Default)]
pub struct ErasedTransformerState {
	pub transformer_map: HashMap<ActionKeyPair, Box<dyn Any + Send + Sync + 'static>>,
}

#[derive(Component, Debug, Default)]
pub struct TransformerOutputCache<S: Signal> {
	pub transformer_map: HashMap<ActionKeyPair, S>,
}
