use rx_core_traits::{Never, Observable, Operator};

use crate::operator::ErrorBoundaryOperator;

pub trait ObservablePipeExtensionErrorBoundary: Observable<OutError = Never> + Sized {
	#[inline]
	fn error_boundary(self) -> <ErrorBoundaryOperator<Self::Out> as Operator>::OutObservable<Self> {
		ErrorBoundaryOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionErrorBoundary for O where O: Observable<OutError = Never> {}
