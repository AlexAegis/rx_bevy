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

#[derive(Default, Debug)]
pub struct TickIndexGenerator {
	current_tick_index: usize,
}

impl TickIndexGenerator {
	pub fn get_next(&mut self) -> TickIndex {
		let tick_id: TickIndex = TickIndex(self.current_tick_index);
		self.current_tick_index += 1;
		tick_id
	}
}

#[cfg(test)]
mod test {
	use std::ops::Deref;

	use crate::TickIndexGenerator;

	#[test]
	fn should_generate_incremental_numbers() {
		let mut id_generator = TickIndexGenerator::default();
		assert_eq!(id_generator.get_next().deref(), &0);
		assert_eq!(id_generator.get_next().deref(), &1);
		assert_eq!(id_generator.get_next().deref(), &2);
		assert_eq!(id_generator.get_next().deref(), &3);
	}

	#[test]
	fn should_display_as_a_number() {
		let mut id_generator = TickIndexGenerator::default();
		let next = id_generator.get_next();
		assert_eq!(format!("{}", next), "0");
	}
}
