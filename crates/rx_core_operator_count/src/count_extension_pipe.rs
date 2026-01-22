use rx_core_common::{Observable, Operator};

use crate::operator::CountOperator;

pub trait ObservablePipeExtensionCount<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [CountOperator]
	///
	/// The `count` operator counts upstream emissions and emits the total once
	/// upstream completes.
	#[inline]
	fn count(
		self,
	) -> <CountOperator<Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		CountOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionCount<'o> for O where O: 'o + Observable + Send + Sync {}
