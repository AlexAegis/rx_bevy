use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FinalizeOperator;

pub trait OperatorComposeExtensionFinalize: Operator + Sized {
	fn finalize<Callback: 'static + Clone + FnOnce() + Send + Sync>(
		self,
		callback: Callback,
	) -> CompositeOperator<Self, FinalizeOperator<Self::Out, Self::OutError, Callback>> {
		CompositeOperator::new(self, FinalizeOperator::new(callback))
	}
}

impl<Op> OperatorComposeExtensionFinalize for Op where Op: Operator {}
