use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};
use derive_where::derive_where;

use crate::Action;

/// Describes what actions are getting activated by what other actions.
/// If the FromAction is a KeyCode, you can think of this as your key-bindings.
#[derive(Component, Deref, DerefMut, Debug)]
#[derive_where(Default)]
pub struct ActionMap<FromAction, ToAction>
where
	FromAction: Action,
	ToAction: Action,
{
	#[deref]
	pub action_map: HashMap<ToAction, FromAction>,
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}
