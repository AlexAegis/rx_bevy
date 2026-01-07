use rx_core_common::{ComposableOperator, Never, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::MapNeverNextOperator;

pub trait OperatorComposeExtensionMapNeverNext: ComposableOperator<Out = Never> + Sized {
	#[inline]
	fn map_never<NextOut: Signal>(
		self,
	) -> CompositeOperator<Self, MapNeverNextOperator<NextOut, Self::OutError>> {
		self.compose_with(MapNeverNextOperator::default())
	}
}

impl<Op> OperatorComposeExtensionMapNeverNext for Op where Op: ComposableOperator<Out = Never> {}
