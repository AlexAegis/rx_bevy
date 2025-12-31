use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapErrorOperator;

pub trait ObservablePipeExtensionMapError: Observable + Sized {
	#[inline]
	fn map_error<
		NextOutError: Signal,
		ErrorMapper: 'static + Fn(Self::OutError) -> NextOutError + Clone + Send + Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <MapErrorOperator<Self::Out, Self::OutError, ErrorMapper, NextOutError> as Operator>::OutObservable<Self>
	{
		MapErrorOperator::new(error_mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionMapError for O where O: Observable {}
