use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, ObservableOutput};

use crate::operator::ExhaustAllOperator;

pub trait OperatorComposeExtensionExhaustAll: ComposableOperator + Sized {
	#[inline]
	fn exhaust_all(self) -> CompositeOperator<Self, ExhaustAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(ExhaustAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionExhaustAll for Op where Op: ComposableOperator {}
