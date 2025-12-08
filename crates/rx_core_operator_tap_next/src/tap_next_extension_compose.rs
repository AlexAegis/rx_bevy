use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::TapNextOperator;

pub trait OperatorComposeExtensionTapNext: Operator + Sized {
	fn tap_next<OnNext: 'static + Fn(&Self::Out) + Clone + Send + Sync>(
		self,
		callback: OnNext,
	) -> CompositeOperator<Self, TapNextOperator<Self::Out, Self::OutError, OnNext>> {
		CompositeOperator::new(self, TapNextOperator::new(callback))
	}
}

impl<Op> OperatorComposeExtensionTapNext for Op where Op: Operator {}
