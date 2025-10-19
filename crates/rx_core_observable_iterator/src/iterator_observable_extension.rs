use rx_core_traits::SubscriptionContext;

use crate::observable::IteratorObservable;

pub trait IntoIteratorObservableExtension: IntoIterator + Clone {
	fn into_observable<Context>(self) -> IteratorObservable<Self, Context>
	where
		Context: SubscriptionContext,
	{
		IteratorObservable::new(self)
	}
}

impl<T> IntoIteratorObservableExtension for T where T: IntoIterator + Clone {}
