use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::FinalizeOperator;

pub trait OperatorComposeExtensionFinalize: ComposableOperator + Sized {
	#[inline]
	fn finalize<Callback: 'static + Clone + FnOnce() + Send + Sync>(
		self,
		callback: Callback,
	) -> CompositeOperator<Self, FinalizeOperator<Self::Out, Self::OutError, Callback>> {
		self.compose_with(FinalizeOperator::new(callback))
	}
}

impl<Op> OperatorComposeExtensionFinalize for Op where Op: ComposableOperator {}
