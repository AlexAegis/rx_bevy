use bevy::prelude::*;

use crate::{Action, Signal};

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

/// Every [Action]'s events are triggered through this wrapper that combines
/// the action, its current [Signal], and the associated [SignalEvent][`crate::SignalEvent`].
///
/// Since [ActionSocket] is a component, and these events are triggered from them,
/// the `.target()` entity on the [Trigger] is **always** defined, and never the
/// [Entity::PLACEHOLDER].
#[derive(Event, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct ActionEvent<A: Action> {
	pub action: A,
	pub signal: A::Signal,
	pub event: <A::Signal as Signal>::Event,
}
