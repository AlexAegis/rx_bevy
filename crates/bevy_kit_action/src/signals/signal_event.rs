use std::fmt::Debug;

use smallvec::SmallVec;

use crate::{ReflectBound, SerializeBound};

use super::{Signal, SignalState};

pub trait SignalEvent<S: Signal>:
	Debug + Send + Sync + Sized + Clone + SerializeBound + ReflectBound
{
	type SignalEventState: SignalEventState;

	fn from_signal_state(signal_state: &SignalState<S>) -> SignalEventVec<Self>;
}

pub type SignalEventVec<S> = SmallVec<[S; 2]>;

pub trait SignalEventState:
	Debug + Default + Clone + Send + Sync + SerializeBound + ReflectBound
{
}

impl SignalEventState for () {}
