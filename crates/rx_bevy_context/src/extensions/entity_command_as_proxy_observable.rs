use bevy_ecs::{entity::Entity, system::EntityCommands};
use rx_core_traits::SignalBound;

use crate::proxy_observable::observable::ProxyObservable;

/// Provides commands for subscription relative to this entity
pub trait EntityCommandAsProxyObservableExtension {
	fn as_proxy_observable<In, InError>(&mut self) -> ProxyObservable<In, InError>
	where
		In: SignalBound + Clone,
		InError: SignalBound + Clone;
}

impl<'a> EntityCommandAsProxyObservableExtension for EntityCommands<'a> {
	fn as_proxy_observable<In, InError>(&mut self) -> ProxyObservable<In, InError>
	where
		In: SignalBound + Clone,
		InError: SignalBound + Clone,
	{
		ProxyObservable::<In, InError>::new(self.id())
	}
}

pub trait EntityAsProxyObservableExtension {
	fn as_observable<In, InError>(&mut self) -> ProxyObservable<In, InError>
	where
		In: SignalBound + Clone,
		InError: SignalBound + Clone;
}

impl EntityAsProxyObservableExtension for Entity {
	fn as_observable<In, InError>(&mut self) -> ProxyObservable<In, InError>
	where
		In: SignalBound + Clone,
		InError: SignalBound + Clone,
	{
		ProxyObservable::<In, InError>::new(*self)
	}
}
