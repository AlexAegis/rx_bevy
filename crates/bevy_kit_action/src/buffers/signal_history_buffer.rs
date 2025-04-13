use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

/// Buffers are only written once per frame, but can be read multiple times,
/// so try to do most work during write
pub trait SignalBuffer<S: Signal>: Default + Send + Sync {
	type BufferOutput;

	fn write<C: Clock>(&mut self, value: S, time: &Res<Time<C>>);
	fn read(&self) -> &Self::BufferOutput;
}

#[derive(Debug, Clone)]
#[derive_where(Default)]
pub struct LastFrameBuffer<S: Signal> {
	pub last_frame_data: Option<S>,
	pub current_signal: S,
}

impl<S: Signal> SignalBuffer<S> for LastFrameBuffer<S> {
	type BufferOutput = Self;
	fn write<C: Clock>(&mut self, value: S, _time: &Res<Time<C>>) {
		self.last_frame_data = Some(self.current_signal);
		self.current_signal = value;
	}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

impl<S: Signal> SignalBuffer<S> for () {
	type BufferOutput = ();

	fn write<C: Clock>(&mut self, _value: S, _time: &Res<Time<C>>) {}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

// TODO: Maybe this could be a more generalized case of SignalLastFrameBuffer
pub struct FrameHistoryBuffer<const L: usize, S: Signal> {
	history: [S; L],
	cursor: usize,
}

impl<const L: usize, S: Signal> Default for FrameHistoryBuffer<L, S> {
	fn default() -> Self {
		Self {
			history: std::array::from_fn(|_| S::default()),
			cursor: 0,
		}
	}
}

impl<const L: usize, S: Signal> SignalBuffer<S> for FrameHistoryBuffer<L, S> {
	// TODO: Change the return type to an array or something that makes sense when reading it, or maybe just forget the cursor business and rotate on write
	type BufferOutput = [S; L];

	fn write<C: Clock>(&mut self, value: S, _time: &Res<Time<C>>) {
		self.cursor += 1;
		if self.cursor == L - 1 {
			self.cursor = 0;
		}

		self.history[self.cursor] = value;
	}

	fn read(&self) -> &Self::BufferOutput {
		&self.history // TODO: Not rotated, maybe could be ignored if it rotates on write
	}
}
