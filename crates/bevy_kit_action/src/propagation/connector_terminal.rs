use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, SignalWriter};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

/// A ConnectorTerminal is like an ActionSocket except it's not triggering
/// anything. It's purpose is to hide the transformation details of a
/// transformer, so that sockets can collect all their inputs in one go.
///
/// TODO: Verify/Implement this comment
/// Since multiple SignalTransformers could exist on the same entity that
/// convert to the same Action, but only one of these terminals can exist,
/// they will be accumulated too one after the other. If this is an undesired
/// behavior, keep them on different entities.
#[derive(Component, Deref, DerefMut, Debug)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component, Default))]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(
	all(feature = "inspector", feature = "reflect"),
	reflect(InspectorOptions)
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct ConnectorTerminal<A: Action> {
	#[deref]
	#[cfg_attr(feature = "serialize", serde(bound(deserialize = "A: Action")))]
	state: HashMap<A, <A as Action>::Signal>,
}

impl<A: Action> SignalWriter<A> for ConnectorTerminal<A> {
	fn write(&mut self, action: &A, value: A::Signal) {
		self.state.entry(*action).insert(value);
	}
}
