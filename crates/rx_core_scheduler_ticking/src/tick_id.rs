use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TickIndex(usize);

impl Display for TickIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for TickIndex {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for TickIndex {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct TickIndexGenerator {
	current_tick_index: usize,
}

impl TickIndexGenerator {
	pub fn get_next(&mut self) -> TickIndex {
		let tick_id: TickIndex = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
