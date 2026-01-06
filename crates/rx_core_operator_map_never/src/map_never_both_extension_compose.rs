use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Never, Signal};

use crate::operator::MapNeverBothOperator;

pub trait OperatorComposeExtensionMapNeverBoth:
	ComposableOperator<Out = Never, OutError = Never> + Sized
{
	#[inline]
	fn map_never_both<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> CompositeOperator<Self, MapNeverBothOperator<NextOut, NextOutError>> {
		self.compose_with(MapNeverBothOperator::default())
	}
}

impl<Op> OperatorComposeExtensionMapNeverBoth for Op where
	Op: ComposableOperator<Out = Never, OutError = Never>
{
}
