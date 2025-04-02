use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};

use derive_where::derive_where;

use crate::{Action, ActionState};

/// Where Actions arrive (Sink?)
/// TODO: It would be so much better to not have these hashmaps here
#[derive(Component, Clone, Debug, Reflect)]
#[derive_where(Default)]
pub struct ActionContext<A: Action> {
	/// TODO(benchmark): Consider that in any given frame maybe only 1 or 2 actions are happening, maybe a simple Vec would be faster!
	pub actions: HashMap<A, ActionState<A>>,
	/// Used to determine action phase transitions
	pub(crate) last_frame_actions: HashMap<A, ActionState<A>>,
	/// Where actions are triggered from. Can point to a gamepad entity,
	/// or the special Keyboard entity, or another entity with an actionContext
	/// as long as there is mapping defined between this context and that
	/// contexts action type, mapping and triggering will occur.
	pub sources: Vec<Entity>,
	_phantom_data_action_key: PhantomData<A>,
}
