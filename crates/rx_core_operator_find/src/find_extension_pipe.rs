use rx_core_traits::{Observable, Operator};

use crate::operator::FindOperator;

pub trait ObservablePipeExtensionFind: Observable + Sized {
	#[inline]
	fn find<Predicate>(
		self,
		predicate: Predicate,
	) -> <FindOperator<Self::Out, Self::OutError, Predicate> as Operator>::OutObservable<Self>
	where
		Predicate: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		FindOperator::new(predicate).operate(self)
	}
}

impl<O> ObservablePipeExtensionFind for O where O: Observable {}
