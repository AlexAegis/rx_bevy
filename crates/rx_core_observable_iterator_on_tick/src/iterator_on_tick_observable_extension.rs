use rx_core_traits::context::SubscriptionContext;

use crate::{IteratorOnTickObservable, OnTickObservableOptions};

pub trait IntoIteratorOnTickObservableExtension: IntoIterator + Clone {
	fn into_observable_on_every_nth_tick<Context>(
		self,
		options: OnTickObservableOptions,
	) -> IteratorOnTickObservable<Self, Context>
	where
		Context: SubscriptionContext,
	{
		IteratorOnTickObservable::new(self, options)
	}
}

impl<T> IntoIteratorOnTickObservableExtension for T where T: IntoIterator + Clone {}
