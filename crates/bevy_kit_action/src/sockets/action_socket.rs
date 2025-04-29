use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;

use crate::{Action, SignalContainer, SocketConnectorTarget};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

// TODO: This needs a way to define how writes accumulate if multiple connectors write into it
#[derive(Component, Deref, DerefMut, Debug, Reflect)]
#[relationship_target(relationship = SocketConnectorTarget::<A>)]
#[cfg_attr(feature = "inspector", derive(InspectorOptions))]
#[cfg_attr(feature = "inspector", reflect(Component, InspectorOptions))]
#[derive_where(Default)]
pub struct ActionSocket<A: Action> {
	#[relationship]
	sources: Vec<Entity>,
	#[deref]
	state: HashMap<A, SignalContainer<<A as Action>::Signal>>,
	/// Normally after every frame, signals reset to their default value
	/// when this option is true, they don't, and a new write is required to
	/// change their signals.
	/// This mainly exist for events that toggle signals like keyboard events.
	pub latching: bool,
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
	) -> impl Iterator<Item = (&A, &SignalContainer<<A as Action>::Signal>)> {
		self.state.iter()
	}

	pub fn iter_signals(&self) -> impl Iterator<Item = (&A, &A::Signal)> {
		self.state
			.iter()
			.map(|(action, container)| (action, &container.signal))
	}

	pub fn write(&mut self, action: &A, value: A::Signal) {
		self.state.entry(*action).or_default().signal = value;
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
