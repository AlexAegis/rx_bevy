use rx_core_common::Signal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FindOperatorError<InError>
where
	InError: Signal,
{
	#[error("FindOperatorError::NoNextObservedBeforeComplete")]
	NoNextObservedBeforeComplete,
	#[error("FindOperatorError::NoMatchObserved")]
	NoMatchObserved,
	#[error(transparent)]
	Upstream(InError),
}
