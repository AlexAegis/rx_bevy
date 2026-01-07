use bevy_ecs::system::EntityCommands;

use rx_bevy_common::RxBevyScheduler;
use rx_core_traits::{SchedulerHandle, Signal};

use crate::observable::ProxyObservable;

/// Provides commands for subscription relative to this entity
pub trait EntityCommandAsProxyObservableExtension {
	fn as_proxy_observable<Out, OutError>(
		&mut self,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> ProxyObservable<Out, OutError>
	where
		Out: Signal + Clone,
		OutError: Signal + Clone;
}

impl<'a> EntityCommandAsProxyObservableExtension for EntityCommands<'a> {
	fn as_proxy_observable<Out, OutError>(
		&mut self,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> ProxyObservable<Out, OutError>
	where
		Out: Signal + Clone,
		OutError: Signal + Clone,
	{
		ProxyObservable::<Out, OutError>::new(self.id(), scheduler)
	}
}
