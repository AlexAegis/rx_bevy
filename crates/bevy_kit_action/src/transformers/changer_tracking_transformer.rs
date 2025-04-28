use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::SignalTransformer;

#[derive(Clone, Reflect)]
#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	buffer: bool,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq, C: Clock> SignalTransformer<C> for ChangeTrackingTransformer<S> {
	type InputSignal = S;
	type OutputSignal = bool;

	fn read(&self) -> Self::OutputSignal {
		self.buffer
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		_time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.buffer = signal == last_frame_input_signal;
	}
}
