use rx_core_common::{Observable, Operator};

use crate::operator::MaterializeOperator;

pub trait ObservablePipeExtensionMaterialize<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn materialize(
		self,
	) -> <MaterializeOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		MaterializeOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMaterialize<'o> for O where O: 'o + Observable + Send + Sync {}
