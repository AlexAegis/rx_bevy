use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, ObservableOutput, Operator};

use crate::operator::MergeAllOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMergeAll: Operator + Sized {
	fn merge_all(self) -> CompositeOperator<Self, MergeAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable<Context = Self::Context>,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		CompositeOperator::new(self, MergeAllOperator::default())
	}
}

impl<T> CompositeOperatorExtensionMergeAll for T where T: Operator {}
