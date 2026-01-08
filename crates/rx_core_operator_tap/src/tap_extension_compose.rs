use rx_core_common::{ComposableOperator, RxObserver};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::TapOperator;

pub trait OperatorComposeExtensionTap: ComposableOperator + Sized {
	#[inline]
	fn tap<TapDestination>(
		self,
		tap_destination: TapDestination,
	) -> CompositeOperator<Self, TapOperator<TapDestination>>
	where
		TapDestination:
			'static + RxObserver<In = Self::Out, InError = Self::OutError> + Clone + Send + Sync,
		Self::Out: Clone,
		Self::OutError: Clone,
	{
		self.compose_with(TapOperator::new(tap_destination))
	}
}

impl<Op> OperatorComposeExtensionTap for Op where Op: ComposableOperator {}
