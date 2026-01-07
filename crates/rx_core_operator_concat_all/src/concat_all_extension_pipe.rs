use rx_core_common::{Observable, ObservableOutput, Operator};

use crate::operator::ConcatAllOperator;

pub trait ObservablePipeExtensionConcatAll<'o>: 'o + Observable + Sized + Send + Sync {
	fn concat_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <ConcatAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator<'o>>::OutObservable<
		Self,
	>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ConcatAllOperator::new(error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionConcatAll<'o> for O where O: 'o + Observable + Send + Sync {}
