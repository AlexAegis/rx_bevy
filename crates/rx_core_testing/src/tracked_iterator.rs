use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

/// A small wrapper around iterators that tracks how many times `next` is
/// called in shared state.
/// Purely for testing purposes.
#[derive(Clone)]
pub struct TrackedIterator<Iterator>
where
	Iterator: IntoIterator,
{
	iterator: Iterator,
	tracking_data: Arc<RwLock<IteratorTrackingData>>,
}

impl<I> From<I> for TrackedIterator<I>
where
	I: Iterator,
{
	fn from(iterator: I) -> Self {
		TrackedIterator {
			iterator,
			tracking_data: Arc::new(RwLock::new(IteratorTrackingData::default())),
		}
	}
}

impl<I> TrackedIterator<I>
where
	I: Iterator,
{
	pub fn new(iterator: I) -> Self {
		iterator.into()
	}

	pub fn get_tracking_data_ref(&self) -> Arc<RwLock<IteratorTrackingData>> {
		self.tracking_data.clone()
	}
}

pub trait IteratorTrackingDataAccess {
	fn read_next_count(&self, instance_id: usize) -> usize;
	fn is_finished(&self, instance_id: usize) -> bool;
}

impl IteratorTrackingDataAccess for Arc<RwLock<IteratorTrackingData>> {
	/// If the iterator or any of its clones is iterated over multiple times, the
	/// `instance_index` is incremented by one.
	/// As this is meant for tests, you have to track this index yourself in your
	/// tests based on the logic you are verifying.
	fn read_next_count(&self, instance_id: usize) -> usize {
		let tracking_data = self.read().unwrap();
		tracking_data.get_instance_data(instance_id).next_count
	}

	fn is_finished(&self, instance_id: usize) -> bool {
		let tracking_data = self.read().unwrap();
		tracking_data.get_instance_data(instance_id).finished
	}
}

impl<I> IntoIterator for TrackedIterator<I>
where
	I: IntoIterator,
{
	type IntoIter = TrackedIntoIter<I>;
	type Item = <TrackedIntoIter<I> as Iterator>::Item;

	fn into_iter(self) -> Self::IntoIter {
		let instance_id = {
			let mut tracking_data = self.tracking_data.write().unwrap();
			tracking_data.create_next_instance()
		};

		TrackedIntoIter {
			instance_id,
			iterator: self.iterator.into_iter(),
			tracking_data: self.tracking_data,
		}
	}
}

pub struct TrackedIntoIter<Iterator>
where
	Iterator: IntoIterator,
{
	instance_id: usize,
	iterator: Iterator::IntoIter,
	tracking_data: Arc<RwLock<IteratorTrackingData>>,
}

impl<I> Iterator for TrackedIntoIter<I>
where
	I: IntoIterator,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		let mut tracking_data = self.tracking_data.write().unwrap();

		let next = self.iterator.next();

		let data = tracking_data.get_instance_data_mut(self.instance_id);
		if next.is_some() {
			data.next_count += 1;
		} else {
			data.finished = true;
		}
		next
	}
}

#[derive(Default)]
pub struct IteratorTrackingData {
	next_instance_id: usize,
	instances: HashMap<usize, IteratorInstanceTrackingData>,
}

impl IteratorTrackingData {
	pub fn create_next_instance(&mut self) -> usize {
		let instance_id = self.next_instance_id;
		self.next_instance_id += 1;
		self.instances
			.insert(instance_id, IteratorInstanceTrackingData::default());
		instance_id
	}

	pub fn get_instance_data(&self, instance_id: usize) -> &IteratorInstanceTrackingData {
		self.instances
			.get(&instance_id)
			.expect("iterator tracking instance {instance_id} does not exist")
	}

	pub fn get_instance_data_mut(
		&mut self,
		instance_id: usize,
	) -> &mut IteratorInstanceTrackingData {
		self.instances
			.get_mut(&instance_id)
			.expect("iterator tracking instance {instance_id} does not exist")
	}
}

#[derive(Default)]
pub struct IteratorInstanceTrackingData {
	next_count: usize,
	finished: bool,
}

#[cfg(test)]
mod test_tracked_iterator {
	use crate::{IteratorTrackingDataAccess, TrackedIterator};

	#[test]
	fn it_should_count_up_all_emissions_and_that_it_has_finished() {
		let tracked_iterator = TrackedIterator::new(1..=3);
		let tracked_data = tracked_iterator.get_tracking_data_ref();
		let iter = tracked_iterator.into_iter();
		assert!(!tracked_data.is_finished(0));
		let collection = iter.collect::<Vec<_>>();

		assert_eq!(collection, vec![1, 2, 3]);
		assert_eq!(tracked_data.read_next_count(0), 3);
		assert!(tracked_data.is_finished(0));
	}

	#[test]
	fn it_should_count_partial_emissions_when_iterated_over_manually() {
		let tracked_iterator = TrackedIterator::new(1..=3);
		let tracked_data = tracked_iterator.get_tracking_data_ref();
		let mut iter = tracked_iterator.into_iter();
		assert!(!tracked_data.is_finished(0));

		let next_1 = iter.next();
		assert_eq!(next_1, Some(1));
		let next_2 = iter.next();
		assert_eq!(next_2, Some(2));

		assert_eq!(tracked_data.read_next_count(0), 2);
		assert!(!tracked_data.is_finished(0));
	}

	#[test]
	fn it_should_count_across_multiple_clones() {
		let tracked_iterator = TrackedIterator::new(1..=3);
		let tracked_data = tracked_iterator.get_tracking_data_ref();
		let iter_1 = tracked_iterator.clone().into_iter();
		assert!(!tracked_data.is_finished(0));
		let collection_1 = iter_1.collect::<Vec<_>>();
		assert_eq!(collection_1, vec![1, 2, 3]);
		let mut iter_2 = tracked_iterator.into_iter();
		let iter_2_next_1 = iter_2.next();
		assert_eq!(iter_2_next_1, Some(1));

		assert_eq!(tracked_data.read_next_count(0), 3);
		assert!(tracked_data.is_finished(0));

		assert_eq!(tracked_data.read_next_count(1), 1);
		assert!(!tracked_data.is_finished(1));
	}
}
