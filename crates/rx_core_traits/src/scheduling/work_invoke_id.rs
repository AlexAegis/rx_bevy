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

#[derive(Default, Debug)]
pub struct WorkInvokeIdGenerator {
	current_index: usize,
}

impl WorkInvokeIdGenerator {
	pub fn get_next(&mut self) -> WorkInvokeId {
		let id = WorkInvokeId(self.current_index);
		self.current_index += 1;
		id
	}
}

#[cfg(test)]
mod test {
	use std::ops::Deref;

	use crate::WorkInvokeIdGenerator;

	#[test]
	fn should_generate_incremental_numbers() {
		let mut id_generator = WorkInvokeIdGenerator::default();
		assert_eq!(id_generator.get_next().deref(), &0);
		assert_eq!(id_generator.get_next().deref(), &1);
		assert_eq!(id_generator.get_next().deref(), &2);
		assert_eq!(id_generator.get_next().deref(), &3);
	}

	#[test]
	fn should_display_as_a_number() {
		let mut id_generator = WorkInvokeIdGenerator::default();
		let next = id_generator.get_next();
		assert_eq!(format!("{}", next), "0");
	}
}
