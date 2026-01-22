use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::CountOperator;

pub trait OperatorComposeExtensionCount: ComposableOperator + Sized {
	/// # [CountOperator]
	///
	/// The `count` operator counts upstream emissions and emits the total once
	/// upstream completes.
	#[inline]
	fn count(self) -> CompositeOperator<Self, CountOperator<Self::Out, Self::OutError>> {
		self.compose_with(CountOperator::default())
	}
}

impl<Op> OperatorComposeExtensionCount for Op where Op: ComposableOperator {}
