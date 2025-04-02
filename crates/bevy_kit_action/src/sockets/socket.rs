//! The big idea: Sockets and Signals

use crate::Action;

/// Stores and optionally transforms signals without having to know about
/// what action wrote it and would read it from it
///
/// Default is required so data can be always written into it.
pub trait SignalTerminal: Default {
	type Input;
	type Output;

	fn write(&mut self, value: Self::Input);
	fn read(&self) -> &Self::Output;
}

/*
pub trait SignalContainer: SignalTransformer<Input = Data, Output = Data> {
	type Data;
}*/

/// An input for an action that will then write it into a container
pub trait SocketInput<A: Action> {
	type Input;

	// TODO: Error handling?
	fn write(&mut self, action: &A, value: Self::Input);
}

pub trait SocketOutput<A: Action> {
	type Output;

	fn read(&self, action: &A) -> Option<&Self::Output>;
}
