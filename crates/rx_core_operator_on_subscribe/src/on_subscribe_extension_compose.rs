use rx_core_common::{ComposableOperator, Subscriber};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::OnSubscribeOperator;

pub trait OperatorComposeExtensionOnSubscribe: ComposableOperator + Sized {
	#[inline]
	fn on_subscribe<OnSubscribe>(
		self,
		on_subscribe: OnSubscribe,
	) -> CompositeOperator<Self, OnSubscribeOperator<OnSubscribe, Self::Out, Self::OutError>>
	where
		OnSubscribe: 'static
			+ FnMut(&mut dyn Subscriber<In = Self::Out, InError = Self::OutError>)
			+ Send
			+ Sync,
	{
		self.compose_with(OnSubscribeOperator::new(on_subscribe))
	}
}

impl<Op> OperatorComposeExtensionOnSubscribe for Op where Op: ComposableOperator {}
