use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::{AdsrOperator, AdsrOperatorOptions};

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionAdsr: Operator<Out = bool> + Sized {
	fn adsr(
		self,
		options: AdsrOperatorOptions,
	) -> CompositeOperator<Self, AdsrOperator<Self::OutError, Self::Context>> {
		CompositeOperator::new(self, AdsrOperator::new(options))
	}
}

impl<Op> CompositeOperatorExtensionAdsr for Op where Op: Operator<Out = bool> {}
