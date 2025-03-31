//! The big idea: Sockets and Signals

use crate::Action;

pub trait InputSocket<A: Action> {
	type Data;

	// TODO: Error handling?
	fn write(&mut self, action: &A, value: &Self::Data);
}

pub trait OutputSocket<A: Action> {
	type Data;

	fn read(&self, action: &A) -> Option<Self::Data>;
}
