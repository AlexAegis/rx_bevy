use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ExhaustAllOperator;

pub trait ObservablePipeExtensionExhaustAll: Observable + Sized {
	#[inline]
	fn exhaust_all(
		self,
	) -> <ExhaustAllOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ExhaustAllOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionExhaustAll for O where O: Observable {}
