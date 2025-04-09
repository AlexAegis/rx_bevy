use bevy::{prelude::*, utils::HashMap};
use derive_where::derive_where;

use crate::{Action, SignalFromTransformer, SignalTransformer};

/// Describes what actions are getting activated by what other actions.
/// If the FromAction is a KeyCode, you can think of this as your key-bindings.
/// TODO: Rename SocketMap
#[derive(Component, Deref, DerefMut, Debug)]
#[derive_where(Default)]
pub struct SocketChannelMap<FromAction, ToAction>
where
	FromAction: Action,
	ToAction: Action,
{
	#[deref]
	pub action_map: HashMap<FromAction, ToAction>,
}

#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct SocketConnector<
	FromAction,
	ToAction,
	SigT = SignalFromTransformer<<FromAction as Action>::Signal, <ToAction as Action>::Signal>,
> where
	FromAction: Action,
	ToAction: Action,
	SigT: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
{
	pub action_map: HashMap<FromAction, ToAction>,
	pub signal_transformer: SigT,
}
