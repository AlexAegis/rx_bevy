use std::collections::VecDeque;

use rx_core_traits::Observable;

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
	#[inline]
	pub fn is_completed(&self) -> bool {
		self.completed && self.is_empty()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.values.is_empty()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.values.len()
	}

	#[inline]
	pub fn push(&mut self, value: O::Out) {
		self.values.push_back(value);
	}

	#[inline]
	pub fn pop(&mut self) -> Option<O::Out> {
		self.values.pop_front()
	}

	#[inline]
	pub fn complete(&mut self) {
		self.completed = true;
	}
}
