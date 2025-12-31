use rx_core_traits::{Observable, Operator};

use crate::operator::FindIndexOperator;

pub trait ObservablePipeExtensionFindIndex<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn find_index<P>(
		self,
		predicate: P,
	) -> <FindIndexOperator<Self::Out, Self::OutError, P> as Operator<'o>>::OutObservable<Self>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		FindIndexOperator::new(predicate).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFindIndex<'o> for O where O: 'o + Observable + Send + Sync {}
