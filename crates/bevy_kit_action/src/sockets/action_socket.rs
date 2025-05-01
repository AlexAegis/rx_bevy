use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, Signal, SignalAggregator, SignalState};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
#[derive_where(Default)]
pub struct ActionSocket<A: Action> {
	#[deref]
	state: HashMap<A, SignalState<<A as Action>::Signal>>,
	/// Normally after every frame, signals reset to their default value
	/// when this option is true, they don't, and a new write is required to
	/// change their signals.
	/// This mainly exist for events that toggle signals like keyboard events.
	pub latching: bool,
}

/// Controls how the socket should behave on subsequent writes, by default
/// TODO: finish comment
#[derive(Component, Debug, Reflect, Default)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
pub enum SocketAccumulationBehavior<A: Action> {
	/// The last write wins!
	#[default]
	Overwrite,
	/// The first write wins!
	Ignore,
	Builtin(<<A as Action>::Signal as Signal>::Accumulator),
}

impl<A: Action> ActionSocket<A> {
	pub fn new_latching() -> Self {
		Self {
			latching: true,
			..Default::default()
		}
	}

	pub fn iter_containers(
		&self,
	) -> impl Iterator<Item = (&A, &SignalState<<A as Action>::Signal>)> {
		self.state.iter()
	}

	pub fn iter_containers_mut(
		&mut self,
	) -> impl Iterator<Item = (&A, &mut SignalState<<A as Action>::Signal>)> {
		self.state.iter_mut()
	}

	pub fn iter_signals(&self) -> impl Iterator<Item = (&A, &A::Signal)> {
		self.state
			.iter()
			.map(|(action, container)| (action, &container.signal))
	}

	pub fn write(
		&mut self,
		action: &A,
		value: A::Signal,
		accumulation_behavior: Option<&SocketAccumulationBehavior<A>>,
	) {
		let signal_state = self.state.entry(*action).or_default();

		if let (Some(accumulation_behavior), true) = (accumulation_behavior, signal_state.written) {
			match accumulation_behavior {
				SocketAccumulationBehavior::Overwrite => {
					signal_state.signal = value;
				}
				SocketAccumulationBehavior::Ignore => {}
				SocketAccumulationBehavior::Builtin(behavior) => {
					signal_state.signal = behavior.combine(signal_state.signal, value);
				}
			}
		} else if signal_state.written {
			let default_accumulator = <A::Signal as Signal>::Accumulator::default();
			signal_state.signal = default_accumulator.combine(signal_state.signal, value);
		} else {
			signal_state.signal = value;
			signal_state.written = true;
		}
	}

	pub fn read(&self, action: &A) -> Option<&A::Signal> {
		self.state
			.get(action)
			.map(|configuration| &configuration.signal)
	}

	pub fn read_last_frame_signal(&self, action: &A) -> Option<&A::Signal> {
		self.state
			.get(action)
			.map(|configuration| &configuration.last_frame_signal)
	}

	pub fn read_or_default(&mut self, action: &A) -> &A::Signal {
		let entry = self.state.entry(*action).or_default();
		&entry.signal
	}

	pub fn read_last_frame_signal_or_default(&mut self, action: &A) -> &A::Signal {
		let entry = self.state.entry(*action).or_default();
		&entry.last_frame_signal
	}
}
