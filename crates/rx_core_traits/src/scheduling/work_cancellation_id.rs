use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkCancellationId(usize);

impl Display for WorkCancellationId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for WorkCancellationId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for WorkCancellationId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct WorkCancellationIdGenerator {
	current_tick_index: usize,
}

impl WorkCancellationIdGenerator {
	pub fn get_next(&mut self) -> WorkCancellationId {
		let tick_id: WorkCancellationId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
