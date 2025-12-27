use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkInvokeId(usize);

impl Display for WorkInvokeId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for WorkInvokeId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for WorkInvokeId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct WorkInvokeIdGenerator {
	current_tick_index: usize,
}

impl WorkInvokeIdGenerator {
	pub fn get_next(&mut self) -> WorkInvokeId {
		let tick_id: WorkInvokeId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
