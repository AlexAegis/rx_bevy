use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapErrorOperator;

pub trait ObservablePipeExtensionMapError<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn map_error<
		NextOutError: Signal,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextOutError + Clone + Send + Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <MapErrorOperator<Self::Out, Self::OutError, ErrorMapper, NextOutError> as Operator<'o>>::OutObservable<Self>
	{
		MapErrorOperator::new(error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMapError<'o> for O where O: 'o + Observable + Send + Sync {}
