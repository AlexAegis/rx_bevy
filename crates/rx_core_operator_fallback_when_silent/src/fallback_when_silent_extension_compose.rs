use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FallbackWhenSilentOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFallbackWhenSilent: Operator + Sized {
	fn fallback_when_silent<Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync>(
		self,
		fallback: Fallback,
	) -> CompositeOperator<
		Self,
		FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, Self::Context>,
	> {
		CompositeOperator::new(self, FallbackWhenSilentOperator::new(fallback))
	}
}

impl<T> CompositeOperatorExtensionFallbackWhenSilent for T where T: Operator {}
