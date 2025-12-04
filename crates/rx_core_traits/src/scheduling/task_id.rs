use std::{fmt::Display, ops::Deref};

/// Unique identifier within a scheduler to cancel active tasks
///
/// Tasks that repeat, or otherwise spawned from the task, will reuse the id,
/// so the source of the task can have full knowledge of the tasks it owns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
