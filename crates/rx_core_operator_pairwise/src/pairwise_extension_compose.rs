use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::PairwiseOperator;

pub trait OperatorComposeExtensionPairwise: ComposableOperator + Sized {
	#[inline]
	fn pairwise(self) -> CompositeOperator<Self, PairwiseOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		self.compose_with(PairwiseOperator::default())
	}
}

impl<Op> OperatorComposeExtensionPairwise for Op where Op: ComposableOperator {}
