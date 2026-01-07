use rx_core_common::{ComposableOperator, Never, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::MapNeverErrorOperator;

pub trait OperatorComposeExtensionMapNeverError:
	ComposableOperator<OutError = Never> + Sized
{
	#[inline]
	fn map_never<NextOutError: Signal>(
		self,
	) -> CompositeOperator<Self, MapNeverErrorOperator<Self::Out, NextOutError>> {
		self.compose_with(MapNeverErrorOperator::default())
	}
}

impl<Op> OperatorComposeExtensionMapNeverError for Op where Op: ComposableOperator<OutError = Never> {}
