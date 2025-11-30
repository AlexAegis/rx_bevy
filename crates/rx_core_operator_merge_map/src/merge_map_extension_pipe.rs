use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::MergeMapOperator;

pub trait ObservablePipeExtensionMergeMap: Observable + Sized {
	fn merge_map<
		NextInnerObservable: Observable<Context = Self::Context> + Signal,
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

impl<O> ObservablePipeExtensionMergeMap for O where O: Observable {}
