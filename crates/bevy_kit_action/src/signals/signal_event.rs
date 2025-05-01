use std::{fmt::Debug, time::Duration};

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};
use smallvec::SmallVec;

use super::{Signal, SignalState};

pub trait SignalEvent<S: Signal>: Debug + Send + Sync + Sized {
	type SignalEventState: SignalEventState;

	fn from_signal_state(signal_state: &SignalState<S>) -> SmallVec<[Self; 1]>;
}

pub trait SignalEventState:
	Debug + Default + Send + Sync + Typed + FromReflect + GetTypeRegistration
{
}

#[derive(Debug, Default, Reflect)]
pub struct SignalBooleanEventState {
	last_activation: Option<Duration>,
}

impl SignalEventState for SignalBooleanEventState {}

impl SignalEventState for () {}

#[derive(Event, Debug)]
pub enum SignalEventBool {
	Activated,
	Deactivated,
}

impl SignalEvent<bool> for SignalEventBool {
	type SignalEventState = SignalBooleanEventState;

	fn from_signal_state(signal_state: &SignalState<bool>) -> SmallVec<[Self; 1]> {
		let mut events = SmallVec::<[Self; 1]>::new();
		if !signal_state.last_frame_signal && signal_state.signal {
			events.push(Self::Activated);
		} else if signal_state.last_frame_signal && !signal_state.signal {
			events.push(Self::Deactivated);
		}

		events
	}
}

#[derive(Event, Debug)]
pub struct SignalNoopEvent;

impl<S: Signal> SignalEvent<S> for SignalNoopEvent {
	type SignalEventState = ();

	fn from_signal_state(_signal_state: &SignalState<S>) -> SmallVec<[Self; 1]> {
		SmallVec::new()
	}
}
