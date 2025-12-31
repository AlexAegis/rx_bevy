use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MulticastSubscriberId(usize);

impl Display for MulticastSubscriberId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl Deref for MulticastSubscriberId {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<usize> for MulticastSubscriberId {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Default, Debug)]
pub struct MulticastSubscriberIdGenerator {
	current_tick_index: usize,
}

impl MulticastSubscriberIdGenerator {
	pub fn get_next(&mut self) -> MulticastSubscriberId {
		let tick_id: MulticastSubscriberId = self.current_tick_index.into();
		self.current_tick_index += 1;
		tick_id
	}
}
