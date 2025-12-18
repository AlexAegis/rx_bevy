use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::StartWithOperator;

pub trait OperatorComposeExtensionStartWith: Operator + Sized {
	fn start_with<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> CompositeOperator<Self, StartWithOperator<OnSubscribe, Self::Out, Self::OutError>>
	where
		OnSubscribe: 'static + FnMut() -> Self::Out + Send + Sync,
	{
		CompositeOperator::new(self, StartWithOperator::new(on_subscribe))
	}
}

impl<Op> OperatorComposeExtensionStartWith for Op where Op: Operator {}
