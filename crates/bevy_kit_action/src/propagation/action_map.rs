use bevy::{ecs::component::Component, utils::HashMap};

use crate::ActionKey;

/// Describes what actions are getting activated by what other actions.
/// If the FromAction is a KeyCode, you can think of this as your key-bindings.
#[derive(Component, Default, Debug)]
pub struct ActionMap<FromAction, ToAction, FromData, ToData>
where
	FromAction: ActionKey<ActionData = FromData>,
	ToAction: ActionKey<ActionData = ToData>,
	ToData: From<FromAction::ActionData>,
{
	pub action_map: HashMap<ToAction, FromAction>,
}
