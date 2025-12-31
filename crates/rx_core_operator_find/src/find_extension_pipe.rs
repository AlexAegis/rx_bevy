use rx_core_traits::{Observable, Operator};

use crate::operator::FindOperator;

pub trait ObservablePipeExtensionFind<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn find<Predicate>(
		self,
		predicate: Predicate,
	) -> <FindOperator<Self::Out, Self::OutError, Predicate> as Operator<'o>>::OutObservable<Self>
	where
		Predicate: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		FindOperator::new(predicate).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFind<'o> for O where O: 'o + Observable + Send + Sync {}
