use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::MergeAllOperator;

pub trait ObservablePipeExtensionMergeAll: Observable + Sized {
	fn merge_all(
		self,
		concurrency_limit: usize,
	) -> Pipe<Self, MergeAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, MergeAllOperator::new(concurrency_limit))
	}
}

impl<O> ObservablePipeExtensionMergeAll for O where O: Observable {}
