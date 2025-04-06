use derive_where::derive_where;

use crate::Signal;

///
/// TODO: Maybe this could be combined with the SignalContainer
pub trait SignalBuffer<S: Signal>: Default {
	type BufferOutput;

	fn push(&mut self, value: S);
	fn read(&self) -> &S;
	fn get_state(&self) -> &Self::BufferOutput;
}

#[derive(Debug)]
#[derive_where(Default)]
pub struct LastFrameBuffer<S: Signal> {
	pub last_frame_data: Option<S>,
	pub current_signal: S,
}

impl<S: Signal> SignalBuffer<S> for LastFrameBuffer<S> {
	type BufferOutput = Self;
	fn push(&mut self, value: S) {
		self.last_frame_data = Some(self.current_signal);
		self.current_signal = value;
	}

	fn read(&self) -> &S {
		&self.current_signal
	}

	fn get_state(&self) -> &Self::BufferOutput {
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

	fn push(&mut self, value: S) {
		self.cursor += 1;
		if self.cursor == L - 1 {
			self.cursor = 0;
		}

		self.history[self.cursor] = value;
	}

	fn read(&self) -> &S {
		&self.history[self.cursor]
	}

	fn get_state(&self) -> &Self::BufferOutput {
		&self.history // TODO: Not rotated, maybe could be ignored if it rotates on write
	}
}
