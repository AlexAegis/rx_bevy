use std::{
	cell::RefCell,
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use bevy::{prelude::*, utils::HashMap};
use derive_where::derive_where;

use crate::{Action, Clock, SignalFromTransformer, SignalTransformer};

#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct SocketConnectorDefaultTransformer<
	C,
	FromAction,
	ToAction,
	Transformer = SignalFromTransformer<
		<FromAction as Action>::Signal,
		<ToAction as Action>::Signal,
	>,
> where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
	C: Clock,
{
	pub default_transformer: Transformer,
	phantom_data_clock: PhantomData<C>,
	phantom_data_from_action: PhantomData<FromAction>,
	phantom_data_to_action: PhantomData<ToAction>,
}

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

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
