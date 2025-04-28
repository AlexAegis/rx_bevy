use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::SignalTransformer;

#[derive(Resource, Clone, Reflect)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	buffer: S,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal, C: Clock> SignalTransformer<C> for IdentitySignalTransformer<S> {
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn read(&self) -> Self::OutputSignal {
		self.buffer
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.buffer = *signal;
	}
}
