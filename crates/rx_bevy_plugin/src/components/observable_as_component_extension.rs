use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_traits::{NotSubject, Observable};

use crate::ObservableComponent;

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait ObservableAsComponentExtension:
	Observable<Context = BevySubscriptionContextProvider, IsSubject = NotSubject> + Send + Sync + Sized
{
	fn into_component(self) -> ObservableComponent<Self>;
}

impl<O> ObservableAsComponentExtension for O
where
	O: Observable<Context = BevySubscriptionContextProvider, IsSubject = NotSubject>
		+ Send
		+ Sync
		+ Sized,
{
	fn into_component(self) -> ObservableComponent<Self> {
		ObservableComponent::new(self)
	}
}
