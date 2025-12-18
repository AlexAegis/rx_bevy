use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::MergeAllOperator;

pub trait ObservablePipeExtensionMergeAll: Observable + Sized {
	#[inline]
	fn merge_all(
		self,
		concurrency_limit: usize,
	) -> <MergeAllOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		MergeAllOperator::new(concurrency_limit).operate(self)
	}
}

impl<O> ObservablePipeExtensionMergeAll for O where O: Observable {}
