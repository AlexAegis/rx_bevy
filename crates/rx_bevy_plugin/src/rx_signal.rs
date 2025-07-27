use bevy_ecs::event::Event;
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::Tick;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RxSignal<In, InError>
where
	In: 'static + Sync + Send + DebugBound,
	InError: 'static + Sync + Send + DebugBound,
{
	Next(In),
	Error(InError),
	Complete,
}

/* DELETE; While it sounds like a good idea, it would require a new scheduler plugin for each signal.
/// Internal events grouped into a single enum simply to avoid having to spawn
/// multiple observer entities for a single subscription.
#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub(crate) enum RxInternal<In, InError>
where
	In: 'static + Sync + Send + DebugBound,
	InError: 'static + Sync + Send + DebugBound,
{
	Signal(RxSignal<In, InError>),
	Tick(Tick),
	Unsubscribe,
}
*/
