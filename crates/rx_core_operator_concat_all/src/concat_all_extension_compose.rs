use rx_core_common::{ComposableOperator, Observable, ObservableOutput};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::ConcatAllOperator;

pub trait OperatorComposeExtensionConcatAll: ComposableOperator + Sized {
	#[inline]
	fn concat_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<Self, ConcatAllOperator<Self::Out, Self::OutError, ErrorMapper>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		self.compose_with(ConcatAllOperator::new(error_mapper))
	}
}

impl<Op> OperatorComposeExtensionConcatAll for Op where Op: ComposableOperator {}
