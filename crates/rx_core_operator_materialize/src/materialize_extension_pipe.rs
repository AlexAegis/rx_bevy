use rx_core_traits::{Observable, Operator};

use crate::operator::MaterializeOperator;

pub trait ObservablePipeExtensionMaterialize: Observable + Sized {
	#[inline]
	fn materialize(
		self,
	) -> <MaterializeOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self> {
		MaterializeOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionMaterialize for O where O: Observable {}
