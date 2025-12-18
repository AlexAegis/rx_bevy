use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::FirstOperator;

pub trait OperatorComposeExtensionFirst: ComposableOperator + Sized {
	#[inline]
	fn first(self) -> CompositeOperator<Self, FirstOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		self.compose_with(FirstOperator::default())
	}
}

impl<Op> OperatorComposeExtensionFirst for Op where Op: ComposableOperator {}
