use rx_core_common::{Observable, ObservableOutput, Operator};

use crate::operator::SwitchAllOperator;

pub trait ObservablePipeExtensionSwitchAll<'o>: 'o + Observable + Sized + Send + Sync {
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
	) -> <SwitchAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator<'o>>::OutObservable<
		Self,
	>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		SwitchAllOperator::new(error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionSwitchAll<'o> for O where O: 'o + Observable + Send + Sync {}
