use bevy::{prelude::*, utils::HashMap};
use derive_where::derive_where;

use crate::{Action, SignalContainer};

#[derive(Debug, Default)]
pub enum SocketConnection {
	#[default]
	This,
	Entity(Entity),
}

#[derive(Component, Deref, DerefMut, Debug)]
#[derive_where(Default)]
pub struct ActionSocket<A: Action> {
	state: HashMap<A, SignalContainer<<A as Action>::Signal>>,
}

impl<A: Action> ActionSocket<A> {
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
