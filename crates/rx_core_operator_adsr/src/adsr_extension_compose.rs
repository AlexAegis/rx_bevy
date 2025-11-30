use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

/// Provides a convenient function to pipe the operator from another operator  
pub trait OperatorComposeExtensionAdsr: Operator<Out = AdsrTrigger> + Sized {
	fn adsr(
		self,
		options: AdsrOperatorOptions,
	) -> CompositeOperator<Self, AdsrOperator<Self::OutError, Self::Context>> {
		CompositeOperator::new(self, AdsrOperator::new(options))
	}
}

impl<Op> OperatorComposeExtensionAdsr for Op where Op: Operator<Out = AdsrTrigger> {}
