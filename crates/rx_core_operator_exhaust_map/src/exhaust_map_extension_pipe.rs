use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ExhaustMapOperator;

pub trait ObservablePipeExtensionExhaustMap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		error_mapper: ErrorMapper,
	) -> <ExhaustMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable> as Operator<'o>>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		ExhaustMapOperator::new(mapper, error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionExhaustMap<'o> for O where O: 'o + Observable + Send + Sync {}
