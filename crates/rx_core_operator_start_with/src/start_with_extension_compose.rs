use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::StartWithOperator;

pub trait OperatorComposeExtensionStartWith: ComposableOperator + Sized {
	#[inline]
	fn start_with<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> CompositeOperator<Self, StartWithOperator<OnSubscribe, Self::Out, Self::OutError>>
	where
		OnSubscribe: 'static + FnMut() -> Self::Out + Send + Sync,
	{
		self.compose_with(StartWithOperator::new(on_subscribe))
	}
}

impl<Op> OperatorComposeExtensionStartWith for Op where Op: ComposableOperator {}
