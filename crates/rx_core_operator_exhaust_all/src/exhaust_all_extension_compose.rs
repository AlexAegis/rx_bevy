use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Observable, ObservableOutput};

use crate::operator::ExhaustAllOperator;

pub trait OperatorComposeExtensionExhaustAll: ComposableOperator + Sized {
	#[inline]
	fn exhaust_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<Self, ExhaustAllOperator<Self::Out, Self::OutError, ErrorMapper>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(ExhaustAllOperator::new(error_mapper))
	}
}

impl<Op> OperatorComposeExtensionExhaustAll for Op where Op: ComposableOperator {}
