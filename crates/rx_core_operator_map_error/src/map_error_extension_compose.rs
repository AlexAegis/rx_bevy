use rx_core_operator_composite::{
	extension_compose::OperatorComposeExtension, operator::CompositeOperator,
};
use rx_core_traits::{ComposableOperator, Signal};

use crate::operator::MapErrorOperator;

pub trait OperatorComposeExtensionMapError: ComposableOperator + Sized {
	#[inline]
	fn map_error<
		NextOutError: Signal,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextOutError + Clone + Send + Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> CompositeOperator<
		Self,
		MapErrorOperator<Self::Out, Self::OutError, ErrorMapper, NextOutError>,
	> {
		self.compose_with(MapErrorOperator::new(error_mapper))
	}
}

impl<Op> OperatorComposeExtensionMapError for Op where Op: ComposableOperator {}
