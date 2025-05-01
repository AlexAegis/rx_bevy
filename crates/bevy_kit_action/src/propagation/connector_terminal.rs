use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, SignalAccumulator, SignalWriter};

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
#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
#[derive_where(Default)]
pub struct ConnectorTerminal<A: Action> {
	#[deref]
	state: HashMap<A, SignalAccumulator<<A as Action>::Signal>>,
}

impl<A: Action> SignalWriter<A> for ConnectorTerminal<A> {
	fn write(&mut self, action: &A, value: A::Signal) {
		let signal_state = self.state.entry(*action).or_default();
		signal_state.signal = value;
	}
}
