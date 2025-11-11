use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::MapIntoOperator;

/// Operator creator function
pub fn map_into<In, InError, Out, OutError, Context>()
-> MapIntoOperator<In, InError, Out, OutError, Context>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Context: SubscriptionContext,
{
	MapIntoOperator::default()
}
