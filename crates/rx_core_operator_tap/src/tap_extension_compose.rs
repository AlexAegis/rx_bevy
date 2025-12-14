use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observer, Operator};

use crate::operator::TapOperator;

pub trait OperatorComposeExtensionTap: Operator + Sized {
	fn tap<TapDestination>(
		self,
		tap_destination: TapDestination,
	) -> CompositeOperator<Self, TapOperator<TapDestination>>
	where
		TapDestination:
			'static + Observer<In = Self::Out, InError = Self::OutError> + Clone + Send + Sync,
		Self::Out: Clone,
		Self::OutError: Clone,
	{
		CompositeOperator::new(self, TapOperator::new(tap_destination))
	}
}

impl<Op> OperatorComposeExtensionTap for Op where Op: Operator {}
