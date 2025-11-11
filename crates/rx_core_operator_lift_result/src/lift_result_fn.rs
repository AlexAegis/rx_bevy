use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::LiftResultOperator;

/// Operator creator function
pub fn lift_result<ResultIn, ResultInError, InError, InErrorToResultError, Context>(
	in_error_to_result_error: InErrorToResultError,
) -> LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError, Context>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
	Context: SubscriptionContext,
{
	LiftResultOperator::new(in_error_to_result_error)
}
