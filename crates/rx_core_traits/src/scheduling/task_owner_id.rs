use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskOwnerId(usize);

impl Display for TaskOwnerId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for TaskOwnerId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for TaskOwnerId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct TaskOwnerIdGenerator {
	current_tick_index: usize,
}

impl TaskOwnerIdGenerator {
	pub fn get_next(&mut self) -> TaskOwnerId {
		let tick_id: TaskOwnerId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
