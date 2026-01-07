use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::SkipOperator;

pub trait OperatorComposeExtensionSkip: ComposableOperator + Sized {
	#[inline]
	fn skip(
		self,
		count: usize,
	) -> CompositeOperator<Self, SkipOperator<Self::Out, Self::OutError>> {
		self.compose_with(SkipOperator::new(count))
	}
}

impl<Op> OperatorComposeExtensionSkip for Op where Op: ComposableOperator {}
