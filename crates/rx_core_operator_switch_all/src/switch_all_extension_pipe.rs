use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::SwitchAllOperator;

pub trait ObservablePipeExtensionSwitchAll: Observable + Sized {
	#[inline]
	fn switch_all(
		self,
	) -> <SwitchAllOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		SwitchAllOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionSwitchAll for O where O: Observable {}
