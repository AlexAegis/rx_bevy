use std::marker::PhantomData;

use bevy::{ecs::component::Component, utils::HashMap};
use derive_where::derive_where;

use crate::Action;

/// Describes what actions are getting activated by what other actions.
/// If the FromAction is a KeyCode, you can think of this as your key-bindings.
/// TODO: SignalMap? It only maps between signals
#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct ActionMap<FromAction, ToAction, FromSignal, ToSignal>
where
	FromAction: Action<Signal = FromSignal>,
	ToAction: Action<Signal = ToSignal>,
	ToSignal: From<FromAction::Signal>,
{
	pub action_map: HashMap<ToSignal, FromSignal>,
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}
