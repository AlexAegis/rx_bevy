use bevy_ecs::event::Event;
use rx_bevy_common_bounds::DebugBound;

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
