use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::EnumerateOperator;

pub trait OperatorComposeExtensionEnumerate: ComposableOperator + Sized {
	#[inline]
	fn enumerate(self) -> CompositeOperator<Self, EnumerateOperator<Self::Out, Self::OutError>> {
		self.compose_with(EnumerateOperator::default())
	}
}

impl<Op> OperatorComposeExtensionEnumerate for Op where Op: ComposableOperator {}
