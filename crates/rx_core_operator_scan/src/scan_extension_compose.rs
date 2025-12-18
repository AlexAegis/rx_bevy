use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Signal};

use crate::operator::ScanOperator;

pub trait OperatorComposeExtensionScan: ComposableOperator + Sized {
	#[inline]
	fn scan<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> CompositeOperator<Self, ScanOperator<Self::Out, Self::OutError, Reducer, NextOut>> {
		self.compose_with(ScanOperator::new(reducer, seed))
	}
}

impl<Op> OperatorComposeExtensionScan for Op where Op: ComposableOperator {}
