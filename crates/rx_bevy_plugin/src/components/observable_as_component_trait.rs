use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_traits::Observable;

use crate::ObservableComponent;

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait ObservableAsComponent:
	Observable<Context = BevySubscriptionContextProvider> + Send + Sync + Sized
{
	fn into_component(self) -> ObservableComponent<Self>;
}

impl<O> ObservableAsComponent for O
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync + Sized,
{
	fn into_component(self) -> ObservableComponent<Self> {
		ObservableComponent::new(self)
	}
}
