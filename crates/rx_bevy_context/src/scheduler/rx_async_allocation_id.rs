use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsyncAllocationId(usize);

impl Display for AsyncAllocationId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for AsyncAllocationId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for AsyncAllocationId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct AsyncAllocationIdGenerator {
	current_index: usize,
}

impl AsyncAllocationIdGenerator {
	pub fn get_next(&mut self) -> AsyncAllocationId {
		let tick_id: AsyncAllocationId = self.current_index.into();
		self.current_index += 1;
		tick_id
	}
}
