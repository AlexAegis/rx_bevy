use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_observable::Tick;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::SignalBound;

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
pub enum RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	Next(In),
	Error(InError),
	Complete,
	Unsubscribe,
	Tick(Tick),
	Add(Entity),
}

impl<In, InError> From<RxNext<In>> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(value: RxNext<In>) -> Self {
		RxSubscriberEvent::Next(value.0)
	}
}

impl<In, InError> From<RxError<InError>> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(value: RxError<InError>) -> Self {
		RxSubscriberEvent::Error(value.0)
	}
}

impl<In, InError> From<RxComplete> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(_value: RxComplete) -> Self {
		RxSubscriberEvent::Complete
	}
}

impl<In, InError> From<RxUnsubscribe> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(_value: RxUnsubscribe) -> Self {
		RxSubscriberEvent::Unsubscribe
	}
}

impl<In, InError> From<RxTick> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(value: RxTick) -> Self {
		RxSubscriberEvent::Tick(value.0)
	}
}

impl<In, InError> From<RxAdd> for RxSubscriberEvent<In, InError>
where
	In: SignalBound + 'static,
	InError: SignalBound + 'static,
{
	fn from(value: RxAdd) -> Self {
		RxSubscriberEvent::Add(value.0)
	}
}
