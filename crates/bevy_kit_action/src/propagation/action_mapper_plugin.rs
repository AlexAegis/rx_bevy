use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionContext, ActionEnvelopePhaseTransition, ActionEnvelopeState, ActionSource,
	ActionState, ActionSystem, ActionSystemFor, Signal,
};

use super::ActionMap;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive_where(Default)]
pub struct ActionMapPlugin<FromAction, ToAction>
where
	FromAction: Action,
	ToAction: Action,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
}

impl<FromAction, ToAction> Plugin for ActionMapPlugin<FromAction, ToAction>
where
	FromAction: Action,
	ToAction: Action,
{
	fn build(&self, app: &mut App) {
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Map
				.after(ActionSystemFor::<FromAction>::Map)
				.after(ActionSystem::Input)
				.before(ActionSystem::Mapped),
		);

		// Actions are triggered backwards compared to mapping
		// TODO: Does it matter? Which is better? This is kinda like bubbling. Should it be a crate feature?
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			map_actions::<FromAction, ToAction>.in_set(ActionSystemFor::<ToAction>::Map),
		);
	}
}

fn map_actions<FromAction, ToAction>(
	mut to_action_context_query: Query<(
		&mut ActionContext<ToAction>,
		&ActionMap<FromAction, ToAction>,
	)>,
	from_action_context_query: Query<&ActionContext<FromAction>>,
) where
	FromAction: Action,
	ToAction: Action,
{
}
