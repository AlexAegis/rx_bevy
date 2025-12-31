use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::ExhaustAllOperator;

pub trait ObservablePipeExtensionExhaustAll<'o>: 'o + Observable + Sized + Send + Sync {
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
	) -> <ExhaustAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		ExhaustAllOperator::new(error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionExhaustAll<'o> for O where O: 'o + Observable + Send + Sync {}
