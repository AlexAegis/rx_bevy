use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::ScanOperator;

pub trait OperatorComposeExtensionScan: Operator + Sized {
	fn scan<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> CompositeOperator<
		Self,
		ScanOperator<Self::Out, Self::OutError, Reducer, NextOut, Self::Context>,
	> {
		CompositeOperator::new(self, ScanOperator::new(reducer, seed))
	}
}

impl<Op> OperatorComposeExtensionScan for Op where Op: Operator {}
