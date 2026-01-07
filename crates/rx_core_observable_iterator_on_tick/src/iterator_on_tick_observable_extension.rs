use rx_core_common::{Scheduler, SchedulerHandle, Signal};

use crate::observable::{IteratorOnTickObservable, OnTickObservableOptions};

pub trait IntoIteratorOnTickObservableExtension: IntoIterator + Clone {
	fn into_observable_on_every_nth_tick<S>(
		self,
		options: OnTickObservableOptions,
		scheduler: SchedulerHandle<S>,
	) -> IteratorOnTickObservable<Self, S>
	where
		Self::Item: Signal,
		S: Scheduler,
	{
		IteratorOnTickObservable::new(self, options, scheduler)
	}
}

impl<T> IntoIteratorOnTickObservableExtension for T where T: IntoIterator + Clone {}
