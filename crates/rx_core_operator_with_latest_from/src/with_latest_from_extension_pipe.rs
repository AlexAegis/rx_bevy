use rx_core_common::{Observable, Operator};

use crate::operator::WithLatestFromOperator;

pub trait ObservablePipeExtensionWithLatestFrom<'o>: 'o + Observable + Sized + Send + Sync {
	/// # [WithLatestFromOperator]
	///
	#[inline]
	fn with_latest_from<InnerObservable>(
		self,
		inner_observable: InnerObservable,
	) -> <WithLatestFromOperator<InnerObservable, Self::Out, Self::OutError> as Operator<'o>>::OutObservable<Self>
	where
		InnerObservable: 'static + Observable<OutError = Self::OutError>,
		InnerObservable::Out: Clone,
	{
		WithLatestFromOperator::new(inner_observable).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionWithLatestFrom<'o> for O where O: 'o + Observable + Send + Sync {}
