use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::SwitchAllOperator;

pub trait ObservablePipeExtensionSwitchAll: Observable + Sized {
	#[inline]
	fn switch_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <SwitchAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		SwitchAllOperator::new(error_mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionSwitchAll for O where O: Observable {}
