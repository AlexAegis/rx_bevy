use bevy::prelude::*;

use crate::{Action, Signal};

/// Every [Action]'s events are triggered through this wrapper that combines
/// the action, its current [Signal], and the associated [SignalEvent][`crate::SignalEvent`].
///
/// Since [ActionSocket] is a component, and these events are triggered from them,
/// the `.target()` entity on the [Trigger] is **always** defined, and never the
/// [Entity::PLACEHOLDER].
#[derive(Event, Debug)]
pub struct ActionEvent<A: Action> {
	pub action: A,
	pub signal: A::Signal,
	pub event: <A::Signal as Signal>::Event,
}
