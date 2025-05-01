use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, Signal, SignalAccumulator, SignalAggregator, SocketAccumulationBehavior};

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

impl<A: Action> ConnectorTerminal<A> {
	// TODO: This write function is a copy on ActionSocket, so maybe do something about that
	pub fn write(
		&mut self,
		action: &A,
		value: A::Signal,
		accumulation_behavior: Option<&SocketAccumulationBehavior<A>>,
	) {
		let signal_container = self.state.entry(*action).or_default();

		if let (Some(accumulation_behavior), true) =
			(accumulation_behavior, signal_container.written)
		{
			match accumulation_behavior {
				SocketAccumulationBehavior::Overwrite => {
					signal_container.signal = value;
				}
				SocketAccumulationBehavior::Ignore => {}
				SocketAccumulationBehavior::Builtin(behavior) => {
					signal_container.signal = behavior.combine(signal_container.signal, value);
				}
			}
		} else if signal_container.written {
			let default_accumulator = <A::Signal as Signal>::Accumulator::default();
			signal_container.signal = default_accumulator.combine(signal_container.signal, value);
		} else {
			signal_container.signal = value;
			signal_container.written = true;
		}
	}
}
