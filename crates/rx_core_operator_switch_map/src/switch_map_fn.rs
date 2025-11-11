use rx_core_traits::{Observable, SignalBound};

use crate::operator::SwitchMapOperator;

/// Operator creator function
pub fn switch_map<In, InError, Switcher, InnerObservable>(
	mapper: Switcher,
) -> SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable + Send + Sync,
{
	SwitchMapOperator::new(mapper)
}
