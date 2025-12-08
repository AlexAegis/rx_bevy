use rx_core_traits::{Scheduler, Signal};

use crate::observable::{IteratorOnTickObservable, OnTickObservableOptions};

pub trait IntoIteratorOnTickObservableExtension: IntoIterator + Clone {
	fn into_observable_on_every_nth_tick<S>(
		self,
		options: OnTickObservableOptions<S>,
	) -> IteratorOnTickObservable<Self, S>
	where
		Self::Item: Signal,
		S: Scheduler,
	{
		IteratorOnTickObservable::new(self, options)
	}
}

impl<T> IntoIteratorOnTickObservableExtension for T where T: IntoIterator + Clone {}
