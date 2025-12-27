use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ExhaustAllOperator;

pub trait ObservablePipeExtensionExhaustAll: Observable + Sized {
	#[inline]
	fn exhaust_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		error_mapper: ErrorMapper,
	) -> <ExhaustAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ExhaustAllOperator::new(error_mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionExhaustAll for O where O: Observable {}
