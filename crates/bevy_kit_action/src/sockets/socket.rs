//! The big idea: Sockets and Signals

pub trait InputSocket {
	type Data;

	// TODO: Error handling?
	fn write(&mut self, value: Self::Data);
}

pub trait OutputSocket {
	type Data;

	fn read(&self) -> Self::Data;
}
