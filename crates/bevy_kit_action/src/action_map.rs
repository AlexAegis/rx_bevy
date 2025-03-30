use bevy::{ecs::component::Component, utils::HashMap};

use crate::ActionKey;

#[derive(Component, Default, Debug)]
pub struct ActionMap<FromAction, ToAction, FromData, ToData>
where
	FromAction: ActionKey<ActionData = FromData>,
	ToAction: ActionKey<ActionData = ToData>,
	ToData: From<FromAction::ActionData>,
{
	pub action_map: HashMap<ToAction, FromAction>,
}
