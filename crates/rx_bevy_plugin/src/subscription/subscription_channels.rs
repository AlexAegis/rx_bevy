use bevy_ecs::event::Event;
use rx_bevy_observable::Tick;

use crate::{RxSignal, RxSubscription, SignalBound};

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]

pub struct RxNext;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxError;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxComplete;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxUnsubscribe;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxAdd;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxTick;

pub trait RxChannel: 'static + Send + Sync + sealed::Sealed {
	type Event<Sub>: Event
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxNext {
	type Event<Sub>
		= RxSignal<Sub::Out, Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxError {
	type Event<Sub>
		= RxSignal<Sub::Out, Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxComplete {
	type Event<Sub>
		= RxSignal<Sub::Out, Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxUnsubscribe {
	type Event<Sub>
		= RxSignal<Sub::Out, Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxAdd {
	type Event<Sub>
		= RxSignal<Sub::Out, Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxTick {
	type Event<Sub>
		= Tick
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

/// 🦭
mod sealed {
	pub trait Sealed {}

	impl Sealed for super::RxNext {}
	impl Sealed for super::RxError {}
	impl Sealed for super::RxComplete {}
	impl Sealed for super::RxUnsubscribe {}
	impl Sealed for super::RxAdd {}
	impl Sealed for super::RxTick {}
}
