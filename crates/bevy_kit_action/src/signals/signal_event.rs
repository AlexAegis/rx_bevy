use std::{fmt::Debug, time::Duration};

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};
use smallvec::SmallVec;

use super::{Signal, SignalState};

pub type SignalEventVec<S> = SmallVec<[S; 2]>;

pub trait SignalEvent<S: Signal>: Debug + Send + Sync + Sized {
	type SignalEventState: SignalEventState;

	fn from_signal_state(signal_state: &SignalState<S>) -> SignalEventVec<Self>;
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
	/// Fired on the signals rising edge, when it just turned from `false` to `true`.
	Activated,
	/// Fired on the signals falling edge, when it just turned from `true` to `false`.
	Deactivated,
	/// Continuous event, fired each frame the signal is true
	Active,
}

impl SignalEvent<bool> for SignalEventBool {
	type SignalEventState = SignalBooleanEventState;

	fn from_signal_state(signal_state: &SignalState<bool>) -> SignalEventVec<Self> {
		let mut events = SmallVec::<[Self; 2]>::new();
		if !signal_state.last_frame_signal && signal_state.signal {
			events.push(Self::Activated);
		} else if signal_state.last_frame_signal && !signal_state.signal {
			events.push(Self::Deactivated);
		}

		if signal_state.signal {
			events.push(Self::Active);
		}

		events
	}
}

#[derive(Event, Debug)]
pub struct SignalNoopEvent;

impl<S: Signal> SignalEvent<S> for SignalNoopEvent {
	type SignalEventState = ();

	fn from_signal_state(_signal_state: &SignalState<S>) -> SignalEventVec<Self> {
		SmallVec::new()
	}
}
