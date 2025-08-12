use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_observable::Tick;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{RxSubscription, SignalBound};

// #[derive(Event, Clone)]
// #[cfg_attr(feature = "debug", derive(Debug))]
// #[cfg_attr(feature = "reflect", derive(Reflect))]
// pub enum RxSignal<In, InError>
// where
// 	In: SignalBound,
// 	InError: SignalBound,
// {
// 	Next(In),
// 	Error(InError),
// 	Complete,
// }

#[derive(Event, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxNext<In>(pub In)
where
	In: SignalBound;

#[derive(Event, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxError<InError>(pub InError)
where
	InError: SignalBound;

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxComplete;

#[derive(Event, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxTick(pub Tick);

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxUnsubscribe;

#[derive(Event, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RxAdd(pub Entity);

/// Internal
#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RxSubscriberEvent<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound + 'static,
	Sub::OutError: SignalBound + 'static,
{
	Next(Sub::Out),
	Error(Sub::OutError),
	Complete,
	Unsubscribe,
	Tick(Tick),
	Add(Entity),
}
