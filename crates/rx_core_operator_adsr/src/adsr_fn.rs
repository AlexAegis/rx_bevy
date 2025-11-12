use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::{AdsrOperator, AdsrOperatorOptions};

/// Operator creator function
pub fn adsr<InError, Context>(options: AdsrOperatorOptions) -> AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext,
{
	AdsrOperator::new(options)
}
