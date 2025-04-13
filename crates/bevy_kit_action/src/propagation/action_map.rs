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

#[derive(Component)]
#[derive_where(Default)]
pub struct SocketConnector<
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
	pub default_transformer_constructor: Option<fn() -> Transformer>,
	/// TODO: Maybe join it into one MappingConfig<Transformer::Config>
	pub(crate) signal_transformer_state: HashMap<FromAction, Transformer>,
	pub action_map: HashMap<FromAction, ToAction>,
	phantom_data_clock: PhantomData<C>, //pub signal_transformer: Transformer,
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
