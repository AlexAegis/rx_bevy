use bevy_ecs::event::Event;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::SignalBound;

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RxSignal<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}
