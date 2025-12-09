use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskInvokeId(usize);

impl Display for TaskInvokeId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for TaskInvokeId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for TaskInvokeId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct TaskInvokeIdGenerator {
	current_tick_index: usize,
}

impl TaskInvokeIdGenerator {
	pub fn get_next(&mut self) -> TaskInvokeId {
		let tick_id: TaskInvokeId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
