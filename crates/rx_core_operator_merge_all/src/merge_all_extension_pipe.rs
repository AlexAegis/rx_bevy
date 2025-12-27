use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::MergeAllOperator;

pub trait ObservablePipeExtensionMergeAll: Observable + Sized {
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
	) -> <MergeAllOperator<Self::Out, Self::OutError, ErrorMapper> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		MergeAllOperator::new(concurrency_limit, error_mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionMergeAll for O where O: Observable {}
