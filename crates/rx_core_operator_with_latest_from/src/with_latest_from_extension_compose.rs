use rx_core_common::{ComposableOperator, Observable};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::WithLatestFromOperator;

pub trait OperatorComposeExtensionWithLatestFrom: ComposableOperator + Sized {
	#[inline]
	fn with_latest_from<InnerObservable>(
		self,
		inner_observable: InnerObservable,
	) -> CompositeOperator<Self, WithLatestFromOperator<InnerObservable, Self::Out, Self::OutError>>
	where
		InnerObservable: 'static + Observable<OutError = Self::OutError>,
		InnerObservable::Out: Clone,
	{
		self.compose_with(WithLatestFromOperator::new(inner_observable))
	}
}

impl<Op> OperatorComposeExtensionWithLatestFrom for Op where Op: ComposableOperator {}
