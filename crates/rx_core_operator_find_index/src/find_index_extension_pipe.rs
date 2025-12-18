use rx_core_traits::{Observable, Operator};

use crate::operator::FindIndexOperator;

pub trait ObservablePipeExtensionFindIndex: Observable + Sized {
	#[inline]
	fn find_index<P>(
		self,
		predicate: P,
	) -> <FindIndexOperator<Self::Out, Self::OutError, P> as Operator>::OutObservable<Self>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		FindIndexOperator::new(predicate).operate(self)
	}
}

impl<O> ObservablePipeExtensionFindIndex for O where O: Observable {}
