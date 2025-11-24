use rx_core_traits::{Observable, SignalBound};

use crate::operator::MergeMapOperator;

/// Operator creator function
pub fn merge_map<In, InError, Switcher, InnerObservable>(
	mapper: Switcher,
) -> MergeMapOperator<In, InError, Switcher, InnerObservable>
where
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable + Send + Sync,
{
	MergeMapOperator::new(mapper)
}
