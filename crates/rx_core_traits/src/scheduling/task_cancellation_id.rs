use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskCancellationId(usize);

impl Display for TaskCancellationId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for TaskCancellationId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for TaskCancellationId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct TaskCancellationIdGenerator {
	current_tick_index: usize,
}

impl TaskCancellationIdGenerator {
	pub fn get_next(&mut self) -> TaskCancellationId {
		let tick_id: TaskCancellationId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
