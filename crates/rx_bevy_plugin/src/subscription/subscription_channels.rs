use bevy_ecs::event::Event;
use rx_core_traits::SignalBound;

use crate::{RxComplete, RxError, RxNext, RxSubscription, RxTick, RxUnsubscribe};

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxChannelNext;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxChannelError;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxChannelComplete;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxChannelUnsubscribe;

//#[derive(Default)]
//#[cfg_attr(feature = "debug", derive(Debug))]
//#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
//pub struct RxChannelAdd;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RxChannelTick;

pub trait RxChannel: 'static + Send + Sync + sealed::Sealed {
	type Event<Sub>: Event
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxChannelNext {
	type Event<Sub>
		= RxNext<Sub::Out>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxChannelError {
	type Event<Sub>
		= RxError<Sub::OutError>
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxChannelComplete {
	type Event<Sub>
		= RxComplete
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

impl RxChannel for RxChannelUnsubscribe {
	type Event<Sub>
		= RxUnsubscribe
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

//impl RxChannel for RxChannelAdd {
//	type Event<Sub>
//		= RxAdd
//	where
//		Sub: RxSubscription,
//		Sub::Out: SignalBound,
//		Sub::OutError: SignalBound;
//}

impl RxChannel for RxChannelTick {
	type Event<Sub>
		= RxTick
	where
		Sub: RxSubscription,
		Sub::Out: SignalBound,
		Sub::OutError: SignalBound;
}

/// 🦭
mod sealed {
	pub trait Sealed {}

	impl Sealed for super::RxChannelNext {}
	impl Sealed for super::RxChannelError {}
	impl Sealed for super::RxChannelComplete {}
	impl Sealed for super::RxChannelUnsubscribe {}
	// impl Sealed for super::RxChannelAdd {}
	impl Sealed for super::RxChannelTick {}
}
