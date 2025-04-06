//! The big idea: Sockets and Signals

use crate::Action;

/// Stores and optionally transforms signals without having to know about
/// what action wrote it and would read it from it
///
/// Default is required so data can be always written into it.
pub trait SignalTerminal:
	SignalTerminalInput<Signal = Self::Input> + SignalTerminalOutput<Signal = Self::Output>
{
	type Input;
	type Output;
}

pub trait SignalTerminalInput: Default + std::fmt::Debug {
	type Signal;

	fn write(&mut self, value: Self::Signal);
}

pub trait SignalTerminalOutput: Default + std::fmt::Debug {
	type Signal;

	fn read(&self) -> &Self::Signal;
}

/*
pub trait SignalContainer: SignalTransformer<Input = Data, Output = Data> {
	type Data;
}*/

/// An input for an action that will then write it into a container
pub trait SocketInput<A: Action> {
	type Signal;

	// TODO: Error handling?
	fn write(&mut self, action: &A, value: Self::Signal);
}

pub trait SocketOutput<A: Action> {
	type Signal;

	fn read(&self, action: &A) -> Option<&Self::Signal>;
}
