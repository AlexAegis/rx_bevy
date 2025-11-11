use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Never, Observable};

use crate::operator::ErrorBoundaryOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionErrorBoundary: Observable<OutError = Never> + Sized {
	fn error_boundary(self) -> Pipe<Self, ErrorBoundaryOperator<Self::Out, Self::Context>> {
		Pipe::new(self, ErrorBoundaryOperator::default())
	}
}

impl<T> ObservableExtensionErrorBoundary for T where T: Observable<OutError = Never> {}
