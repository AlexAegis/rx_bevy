use rx_core_common::Signal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FindIndexOperatorError<InError>
where
	InError: Signal,
{
	#[error("FindIndexOperatorError::NoNextObservedBeforeComplete")]
	NoNextObservedBeforeComplete,
	#[error("FindIndexOperatorError::NoMatchObserved")]
	NoMatchObserved,
	#[error(transparent)]
	Upstream(InError),
}
