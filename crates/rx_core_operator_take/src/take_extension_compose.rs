use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::TakeOperator;

pub trait OperatorComposeExtensionTake: ComposableOperator + Sized {
	#[inline]
	fn take(
		self,
		count: usize,
	) -> CompositeOperator<Self, TakeOperator<Self::Out, Self::OutError>> {
		self.compose_with(TakeOperator::new(count))
	}
}

impl<Op> OperatorComposeExtensionTake for Op where Op: ComposableOperator {}
