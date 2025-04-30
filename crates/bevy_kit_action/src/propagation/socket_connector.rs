use std::marker::PhantomData;

use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, Clock, SignalTransformer, SocketConnections};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

/// Optional component, when present next to a Connector with the same
/// FromAction type, will use the targeted entity to find the socket
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct SocketConnectorSource<A: Action> {
	#[deref]
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

/// Optional component, when present next to a Connector with the same
/// ToAction type, will use the targeted entity to find the socket
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[relationship(relationship_target = SocketConnections<A>)]
pub struct SocketConnectorTarget<A: Action> {
	#[deref]
	#[relationship]
	target: Entity,
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> SocketConnectorTarget<A> {
	pub fn new(target: Entity) -> Self {
		Self {
			target,
			_phantom_data_action: PhantomData,
		}
	}
}

#[derive(Component, Reflect)]
#[derive_where(Default)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
pub struct SocketConnector<C, FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
	C: Clock,
{
	#[reflect(ignore)]
	pub default_transformer_constructor: Option<fn() -> Transformer>,
	pub(crate) signal_transformer_state: HashMap<ToAction, Transformer>,
	pub action_map: HashMap<FromAction, ToAction>,
	#[reflect(ignore)]
	phantom_data_clock: PhantomData<C>,
}

impl<C, FromAction, ToAction, Transformer> SocketConnector<C, FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
	C: Clock,
{
	#[must_use]
	pub fn new(default_transformer: fn() -> Transformer) -> Self {
		Self {
			default_transformer_constructor: Some(default_transformer),
			..Default::default()
		}
	}
}
