use std::{fmt::Display, ops::Deref};

/// Unique identifier within a scheduler to cancel active tasks
///
/// Tasks that repeat, or otherwise spawned from the task, will reuse the id,
/// so the source of the task can have full knowledge of the tasks it owns.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(usize);

impl Display for TaskId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for TaskId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for TaskId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct TaskIdGenerator {
	current_tick_index: usize,
}

impl TaskIdGenerator {
	pub fn get_next(&mut self) -> TaskId {
		let tick_id: TaskId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
