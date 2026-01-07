use rx_core_common::{Observable, Operator};

use crate::operator::PairwiseOperator;

pub trait ObservablePipeExtensionPairwise<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [PairwiseOperator]
	///
	/// Pairs up upstream next emissions and emits them in an array.
	/// Since the first upstream emission can't yet be paired up with anything,
	/// the first downstream emission happens on the second upstream emission.
	///
	#[inline]
	fn pairwise(
		self,
	) -> <PairwiseOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Clone,
	{
		PairwiseOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionPairwise<'o> for O where O: 'o + Observable + Send + Sync {}
