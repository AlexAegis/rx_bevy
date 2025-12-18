use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::EndWithOperator;

pub trait OperatorComposeExtensionEndWith: ComposableOperator + Sized {
	#[inline]
	fn end_with(
		self,
		end_with: Self::Out,
	) -> CompositeOperator<Self, EndWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		self.compose_with(EndWithOperator::new(end_with))
	}
}

impl<Op> OperatorComposeExtensionEndWith for Op where Op: ComposableOperator {}
