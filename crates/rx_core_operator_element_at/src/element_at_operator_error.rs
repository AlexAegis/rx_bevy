use rx_core_common::Signal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ElementAtOperatorError<InError>
where
	InError: Signal,
{
	#[error(
		"ElementAtOperatorError::IndexOutOfRange requested_index={requested_index} observed_nexts={observed_nexts}"
	)]
	IndexOutOfRange {
		requested_index: usize,
		observed_nexts: usize,
	},
	#[error(transparent)]
	Upstream(InError),
}
