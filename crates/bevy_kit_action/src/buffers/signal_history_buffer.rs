use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

/// Buffers are only written once per frame, but can be read multiple times,
/// so try to do most work during write
pub trait SignalBuffer: Default + Send + Sync {
	// TODO: Delete?
	type BufferOutput;
	type InputSignal: Signal;
	type OutputSignal: Signal;

	/// TODO: benchmark, measure if it makes a difference when input/outputsignal is a ref vs copied, signals should be small, a Vec3 at most
	fn write<C: Clock>(
		&mut self,
		input_signal: Self::InputSignal,
		time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		last_frame_output_signal: &Self::OutputSignal,
	);
	fn read(&self) -> &Self::BufferOutput;
}

#[derive(Debug, Clone, Reflect)]
#[derive_where(Default)]
pub struct SimpleSignalBuffer<InputSignal: Signal, OutputSignal: Signal> {
	pub signal: InputSignal,
	#[reflect(ignore)]
	_phantom_data_output_signal: PhantomData<OutputSignal>,
}

impl<InputSignal: Signal, OutputSignal: Signal> SignalBuffer
	for SimpleSignalBuffer<InputSignal, OutputSignal>
{
	type BufferOutput = Self;
	type InputSignal = InputSignal;
	type OutputSignal = OutputSignal;

	fn write<C: Clock>(
		&mut self,
		value: Self::InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.signal = value;
	}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

impl<InputSignal: Signal, OutputSignal: Signal> SignalBuffer
	for LastFrameBuffer<InputSignal, OutputSignal>
{
	type BufferOutput = Self;
	type InputSignal = InputSignal;
	type OutputSignal = OutputSignal;
	fn write<C: Clock>(
		&mut self,
		value: Self::InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.last_frame_input_signal = Some(self.current_signal);
		self.current_signal = value;
	}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

#[derive(Debug, Clone, Reflect)]
#[derive_where(Default)]
pub struct LastFrameBuffer<InputSignal: Signal, OutputSignal: Signal> {
	pub last_frame_input_signal: Option<InputSignal>,
	pub last_frame_output_signal: Option<OutputSignal>,
	pub current_signal: InputSignal,
}

// TODO: Maybe this could be a more generalized case of SignalLastFrameBuffer
pub struct FrameHistoryBuffer<const L: usize, InputSignal: Signal, OutputSignal: Signal> {
	input_history: [InputSignal; L],
	output_history: [OutputSignal; L],
	cursor: usize,
}

impl<const L: usize, InputSignal: Signal, OutputSignal: Signal> Default
	for FrameHistoryBuffer<L, InputSignal, OutputSignal>
{
	fn default() -> Self {
		Self {
			input_history: std::array::from_fn(|_| InputSignal::default()),
			output_history: std::array::from_fn(|_| OutputSignal::default()),
			cursor: 0,
		}
	}
}

impl<const L: usize, InputSignal: Signal, OutputSignal: Signal> SignalBuffer
	for FrameHistoryBuffer<L, InputSignal, OutputSignal>
{
	// TODO: Change the return type to an array or something that makes sense when reading it, or maybe just forget the cursor business and rotate on write
	type BufferOutput = [InputSignal; L];
	type InputSignal = InputSignal;
	type OutputSignal = OutputSignal;

	fn write<C: Clock>(
		&mut self,
		value: InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.cursor += 1;
		if self.cursor == L - 1 {
			self.cursor = 0;
		}

		self.input_history[self.cursor] = value;
	}

	fn read(&self) -> &Self::BufferOutput {
		&self.input_history // TODO: Not rotated, maybe could be ignored if it rotates on write
	}
}
