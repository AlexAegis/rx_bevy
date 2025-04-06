use std::marker::PhantomData;

use bevy::{input::InputSystem, prelude::*};
use derive_where::derive_where;

use crate::{Action, ActionSystem};

/// To set up system ordering
#[derive_where(Default)]
pub struct ActionResetPlugin<A>
where
	A: Action,
{
	_phantom_data_action: PhantomData<A>,
}

impl<A> Plugin for ActionResetPlugin<A>
where
	A: Action,
{
	fn build(&self, app: &mut App) {
		#[cfg(feature = "reflect")]
		{
			app.register_type::<A>();
		}

		// Clear actions before bevy would emit the current ones for this frame
		app.configure_sets(PreUpdate, ActionSystem::Reset.before(InputSystem));

		// app.add_systems(PreUpdate, reset_actions::<A>.in_set(ActionSystem::Reset));
	}
}
/*
fn reset_actions<A>(action_buffer_query: Query<&ActionBuffer<A>>)
where
	A: Action,
{
	for action_context in action_buffer_query.iter() {
		action_context.last_frame_actions = std::mem::take(&mut action_context.actions);
	}
}
*/
