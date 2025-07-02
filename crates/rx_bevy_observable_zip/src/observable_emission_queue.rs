use std::collections::VecDeque;

use rx_bevy_observable::Observable;

pub struct ObservableEmissionQueue<O>
where
	O: Observable,
{
	values: VecDeque<O::Out>,
	completed: bool,
}

impl<O> Default for ObservableEmissionQueue<O>
where
	O: Observable,
{
	fn default() -> Self {
		Self {
			values: VecDeque::with_capacity(2),
			completed: false,
		}
	}
}

impl<O> ObservableEmissionQueue<O>
where
	O: Observable,
{
	pub fn is_completed(&self) -> bool {
		self.completed && self.values.len() == 0
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}

	pub fn push(&mut self, value: O::Out) {
		self.values.push_back(value);
	}

	pub fn pop(&mut self) -> Option<O::Out> {
		self.values.pop_front()
	}

	pub fn complete(&mut self) {
		self.completed = true;
	}
}
