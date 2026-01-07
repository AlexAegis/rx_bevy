use rx_core_common::{ComposableOperator, Subscriber};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::OnNextOperator;

pub trait OperatorComposeExtensionOnNext: ComposableOperator + Sized {
	#[inline]
	fn on_next<OnNext>(
		self,
		on_next: OnNext,
	) -> CompositeOperator<Self, OnNextOperator<OnNext, Self::Out, Self::OutError>>
	where
		OnNext: 'static
			+ FnMut(&Self::Out, &mut dyn Subscriber<In = Self::Out, InError = Self::OutError>) -> bool
			+ Send
			+ Sync
			+ Clone,
	{
		self.compose_with(OnNextOperator::new(on_next))
	}
}

impl<Op> OperatorComposeExtensionOnNext for Op where Op: ComposableOperator {}
