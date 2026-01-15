use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::IntoResultOperator;

pub trait OperatorComposeExtensionIntoResult: ComposableOperator + Sized {
	/// [IntoResultOperator]
	///
	/// Error handling operator. Captures upstream values and errors, and forwards
	/// them downstream as a `Result`.
	#[inline]
	fn into_result(self) -> CompositeOperator<Self, IntoResultOperator<Self::Out, Self::OutError>> {
		self.compose_with(IntoResultOperator::default())
	}
}

impl<Op> OperatorComposeExtensionIntoResult for Op where Op: ComposableOperator {}
