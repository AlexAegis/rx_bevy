use rx_core_traits::{Observable, SignalBound};

use crate::operator::SwitchAllOperator;

/// Operator creator function
pub fn switch_all<In, InError>() -> SwitchAllOperator<In, InError>
where
	In: SignalBound + Observable,
	InError: SignalBound + Into<In::OutError>,
{
	SwitchAllOperator::default()
}
