use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Never, Observable};

use crate::operator::ErrorBoundaryOperator;

pub trait ObservablePipeExtensionErrorBoundary: Observable<OutError = Never> + Sized {
	fn error_boundary(self) -> Pipe<Self, ErrorBoundaryOperator<Self::Out>> {
		Pipe::new(self, ErrorBoundaryOperator::default())
	}
}

impl<O> ObservablePipeExtensionErrorBoundary for O where O: Observable<OutError = Never> {}
