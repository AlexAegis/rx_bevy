use rx_core_traits::Signal;

use crate::operator::IdentityOperator;

/// It creates an IdentityOperator to easily define the input types of a
/// composite operator.
#[inline]
pub fn compose_operator<In, InError>() -> IdentityOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	IdentityOperator::default()
}
