use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::MergeMapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMergeMap: Observable + Sized {
	fn merge_map<
		NextInnerObservable: 'static + Observable<Context = Self::Context> + Send + Sync,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> Pipe<Self, MergeMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, MergeMapOperator::new(switcher))
	}
}

impl<T> ObservableExtensionMergeMap for T where T: Observable {}
