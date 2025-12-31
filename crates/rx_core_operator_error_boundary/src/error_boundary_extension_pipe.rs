use rx_core_traits::{Never, Observable, Operator};

use crate::operator::ErrorBoundaryOperator;

pub trait ObservablePipeExtensionErrorBoundary<'o>:
	'o + Observable<OutError = Never> + Sized + Send + Sync
{
	#[inline]
	fn error_boundary(
		self,
	) -> <ErrorBoundaryOperator<Self::Out> as Operator<'o>>::OutObservable<Self> {
		ErrorBoundaryOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionErrorBoundary<'o> for O where
	O: 'o + Observable<OutError = Never> + Send + Sync
{
}
