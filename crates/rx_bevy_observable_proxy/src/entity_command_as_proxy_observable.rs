use bevy_ecs::{schedule::ScheduleLabel, system::EntityCommands};

use rx_bevy_common::Clock;
use rx_core_traits::SignalBound;

use crate::observable::ProxyObservable;

/// Provides commands for subscription relative to this entity
pub trait EntityCommandAsProxyObservableExtension {
	fn as_proxy_observable<Out, OutError, S, C>(&mut self) -> ProxyObservable<Out, OutError, S, C>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
		S: ScheduleLabel,
		C: Clock;
}

impl<'a> EntityCommandAsProxyObservableExtension for EntityCommands<'a> {
	fn as_proxy_observable<Out, OutError, S, C>(&mut self) -> ProxyObservable<Out, OutError, S, C>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
		S: ScheduleLabel,
		C: Clock,
	{
		ProxyObservable::<Out, OutError, S, C>::new(self.id())
	}
}
