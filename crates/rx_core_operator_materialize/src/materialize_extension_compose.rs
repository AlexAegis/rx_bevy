use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::MaterializeOperator;

pub trait OperatorComposeExtensionMaterialize: ComposableOperator + Sized {
	#[inline]
	fn materialize(
		self,
	) -> CompositeOperator<Self, MaterializeOperator<Self::Out, Self::OutError>> {
		self.compose_with(MaterializeOperator::default())
	}
}

impl<Op> OperatorComposeExtensionMaterialize for Op where Op: ComposableOperator {}
