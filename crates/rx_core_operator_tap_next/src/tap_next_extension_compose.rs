use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::TapNextOperator;

pub trait OperatorComposeExtensionTapNext: ComposableOperator + Sized {
	#[inline]
	fn tap_next<OnNext: 'static + Fn(&Self::Out) + Clone + Send + Sync>(
		self,
		callback: OnNext,
	) -> CompositeOperator<Self, TapNextOperator<Self::Out, Self::OutError, OnNext>> {
		self.compose_with(TapNextOperator::new(callback))
	}
}

impl<Op> OperatorComposeExtensionTapNext for Op where Op: ComposableOperator {}
