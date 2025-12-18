use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, ObservableOutput};

use crate::operator::ConcatAllOperator;

pub trait OperatorComposeExtensionConcatAll: ComposableOperator + Sized {
	#[inline]
	fn concat_all(self) -> CompositeOperator<Self, ConcatAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(ConcatAllOperator::default())
	}
}

impl<Op> OperatorComposeExtensionConcatAll for Op where Op: ComposableOperator {}
