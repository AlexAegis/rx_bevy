use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::IntoResultOperator;

pub trait OperatorComposeExtensionIntoResult: ComposableOperator + Sized {
	#[inline]
	fn into_result(self) -> CompositeOperator<Self, IntoResultOperator<Self::Out, Self::OutError>> {
		self.compose_with(IntoResultOperator::default())
	}
}

impl<Op> OperatorComposeExtensionIntoResult for Op where Op: ComposableOperator {}
