use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, SignalBound};

use crate::operator::ScanOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionScan: Operator + Sized {
	fn scan<
		NextOut: SignalBound + Clone,
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

impl<T> CompositeOperatorExtensionScan for T where T: Operator {}
