use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ConcatAllOperator;

pub trait ObservablePipeExtensionConcatAll: Observable + Sized {
	fn concat_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <ConcatAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ConcatAllOperator::new(error_mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionConcatAll for O where O: Observable {}
