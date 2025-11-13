use bevy_ecs::system::EntityCommands;

use rx_core_traits::SignalBound;

use crate::proxy::observable::ProxyObservable;

/// Provides commands for subscription relative to this entity
pub trait EntityCommandAsProxyObservableExtension {
	fn as_proxy_observable<Out, OutError>(&mut self) -> ProxyObservable<Out, OutError>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone;
}

impl<'a> EntityCommandAsProxyObservableExtension for EntityCommands<'a> {
	fn as_proxy_observable<Out, OutError>(&mut self) -> ProxyObservable<Out, OutError>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
	{
		ProxyObservable::<Out, OutError>::new(self.id())
	}
}
