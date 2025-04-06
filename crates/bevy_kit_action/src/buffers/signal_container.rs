use derive_where::derive_where;

use crate::Signal;

use super::SignalBuffer;

/// The most trivial signal buffer, holds a single value
#[derive(Debug, Default)]
pub struct SignalContainer<S: Signal> {
	pub signal: S,
}

impl<S: Signal> SignalBuffer<S> for SignalContainer<S> {
	type BufferOutput = S;
	fn push(&mut self, value: S) {
		self.signal = value;
	}
	fn read(&self) -> &Self::BufferOutput {
		&self.signal
	}
	fn get_state(&self) -> &Self::BufferOutput {
		&self.signal
	}
}

/*
impl<S> SignalTerminalOutput for SignalContainer<S>
where
	S: Signal,
{
	type Signal = Option<S>;

	fn read(&self) -> &Self::Signal {
		&self.state
	}
}

impl<S> SignalTerminalInput for SignalContainer<S>
where
	S: Signal,
{
	type Signal = Option<S>;

	fn write(&mut self, value: Self::Signal) {
		self.state = value;
	}
}
*/
/*
impl<T> SignalTerminal for T
where
	T: Signal + Default,
{
	type Input = Self;
	type Output = Self;

	fn read<'a>(&'a self) -> &'a Self::Output {
		&self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}

impl<T> SignalTerminal for Option<T>
where
	T: Signal,
{
	type Input = Self;
	type Output = Self;

	fn read<'a>(&'a self) -> &'a Self::Output {
		&self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}*//*

/// TODO: Not sure if this is actually helping anything, it just ignores action
impl<A, T: SignalTerminal<Output = A::Signal>> SocketOutput<A> for T
where
	A: Action,
{
	type Output = A::Signal;

	fn read(&self, _action: &A) -> Option<&Self::Output> {
		Some(self.read())
	}
}

impl<A, T: SignalTerminal<Input = A::Signal>> SocketInput<A> for T
where
	A: Action,
{
	type Input = A::Signal;

	fn write(&mut self, _action: &A, value: &Self::Input) {
		self.write(value);
	}
}*/

/*
// TODO: Could the HashMap<A, SocketState> pattern be extracted for simpler socket implementations?
/// Simple on/off socket
#[derive(Default, Debug)]
pub struct BooleanDataContainer {
	pub state: bool,
}

impl SignalTerminal for BooleanDataContainer {
	type Input = bool;
	type Output = bool;

	fn read(&self) -> Self::Input {
		self.state
	}

	fn write(&mut self, value: &Self::Input) {
		self.state = *value;
	}
}*/
/*
impl SignalTerminal for bool {
	type Input = bool;
	type Output = bool;

	fn read(&self) -> &Self::Input {
		self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}*/
