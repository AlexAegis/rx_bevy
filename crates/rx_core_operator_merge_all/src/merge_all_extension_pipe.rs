use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::MergeAllOperator;

pub trait ObservablePipeExtensionMergeAll<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn merge_all<
		ErrorMapper: 'static
			+ Fn(Self::OutError) -> <Self::Out as ObservableOutput>::OutError
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		concurrency_limit: usize,
		error_mapper: ErrorMapper,
	) -> <MergeAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator<'o>>::OutObservable<
		Self,
	>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		MergeAllOperator::new(concurrency_limit, error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMergeAll<'o> for O where O: 'o + Observable + Send + Sync {}
