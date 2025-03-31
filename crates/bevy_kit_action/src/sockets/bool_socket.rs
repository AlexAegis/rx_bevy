use super::{InputSocket, OutputSocket};

/// Simple on/off socket
pub struct BooleanSocket {
	value: bool,
}

impl OutputSocket for BooleanSocket {
	type Data = bool;

	fn read(&self) -> Self::Data {
		self.value
	}
}

impl InputSocket for BooleanSocket {
	type Data = bool;

	fn write(&mut self, value: Self::Data) {
		self.value = value;
	}
}
