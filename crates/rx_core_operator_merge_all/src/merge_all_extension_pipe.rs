use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::MergeAllOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMergeAll: Observable + Sized {
	fn merge_all(self) -> Pipe<Self, MergeAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable<Context = Self::Context>,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, MergeAllOperator::default())
	}
}

impl<T> ObservableExtensionMergeAll for T where T: Observable {}
