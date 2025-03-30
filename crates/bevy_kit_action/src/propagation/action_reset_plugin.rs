use std::marker::PhantomData;

use bevy::{input::InputSystem, prelude::*};
use derive_where::derive_where;

use crate::{ActionContext, ActionKey, ActionSystem};

/// To set up system ordering
#[derive_where(Default)]
pub struct ActionResetPlugin<Action, Data = <Action as ActionKey>::ActionData>
where
	Action: ActionKey<ActionData = Data>,
{
	_phantom_data_action: PhantomData<Action>,
}

impl<Action, Data> Plugin for ActionResetPlugin<Action, Data>
where
	Action: ActionKey<ActionData = Data>,
	Data: 'static,
{
	fn build(&self, app: &mut App) {
		#[cfg(feature = "reflect")]
		{
			app.register_type::<Action>();
		}

		// Clear actions before bevy would emit the current ones for this frame
		app.configure_sets(PreUpdate, ActionSystem::Reset.before(InputSystem));

		app.add_systems(
			PreUpdate,
			reset_actions::<Action, Data>.in_set(ActionSystem::Reset),
		);
	}
}

fn reset_actions<Action, Data>(mut action_context_query: Query<&mut ActionContext<Action>>)
where
	Action: ActionKey<ActionData = Data>,
	Data: 'static,
{
	for mut action_context in action_context_query.iter_mut() {
		action_context.last_frame_actions = std::mem::take(&mut action_context.actions);
	}
}
