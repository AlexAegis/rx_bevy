use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::StartWithOperator;

pub trait OperatorComposeExtensionStartWith: ComposableOperator + Sized {
	#[inline]
	fn start_with(
		self,
		start_with: Self::Out,
	) -> CompositeOperator<Self, StartWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		self.compose_with(StartWithOperator::new(start_with))
	}
}

impl<Op> OperatorComposeExtensionStartWith for Op where Op: ComposableOperator {}
