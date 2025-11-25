use rx_core_traits::{Observable, PrimaryCategoryObservable};

use crate::{BevySubscriptionContextProvider, ObservableComponent};

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait ObservableAsComponentExtension:
	Observable<
		Context = BevySubscriptionContextProvider,
		PrimaryCategory = PrimaryCategoryObservable,
	> + Send
	+ Sync
	+ Sized
{
	fn into_component(self) -> ObservableComponent<Self>;
}

impl<O> ObservableAsComponentExtension for O
where
	O: Observable<
			Context = BevySubscriptionContextProvider,
			PrimaryCategory = PrimaryCategoryObservable,
		> + Send
		+ Sync
		+ Sized,
{
	fn into_component(self) -> ObservableComponent<Self> {
		ObservableComponent::new(self)
	}
}
