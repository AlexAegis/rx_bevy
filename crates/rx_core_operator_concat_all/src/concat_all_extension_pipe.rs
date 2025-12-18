use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ConcatAllOperator;

pub trait ObservablePipeExtensionConcatAll: Observable + Sized {
	fn concat_all(
		self,
	) -> <ConcatAllOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ConcatAllOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionConcatAll for O where O: Observable {}
