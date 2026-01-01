use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::BufferCountOperator;

pub trait OperatorComposeExtensionBufferCount: ComposableOperator + Sized {
	#[inline]
	fn buffer_count(
		self,
		buffer_size: usize,
	) -> CompositeOperator<Self, BufferCountOperator<Self::Out, Self::OutError>> {
		self.compose_with(BufferCountOperator::new(buffer_size))
	}
}

impl<Op> OperatorComposeExtensionBufferCount for Op where Op: ComposableOperator {}
