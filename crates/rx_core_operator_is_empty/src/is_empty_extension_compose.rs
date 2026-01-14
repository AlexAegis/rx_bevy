use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::IsEmptyOperator;

pub trait OperatorComposeExtensionIsEmpty: ComposableOperator + Sized {
	#[allow(clippy::wrong_self_convention, reason = "This is an operator")]
	#[inline]
	fn is_empty(self) -> CompositeOperator<Self, IsEmptyOperator<Self::Out, Self::OutError>> {
		self.compose_with(IsEmptyOperator::default())
	}
}

impl<Op> OperatorComposeExtensionIsEmpty for Op where Op: ComposableOperator {}
