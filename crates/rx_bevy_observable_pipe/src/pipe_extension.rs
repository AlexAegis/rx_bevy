use rx_bevy_core::{Observable, Operator};

use crate::Pipe;

/// Provides a convenient function to pipe an operator from an observable
/// It's most useful for composite operators
pub trait ObservableExtensionPipe: Observable + Sized {
	fn pipe<Op>(self, operator: Op) -> Pipe<Self, Op>
	where
		Self: Sized,
		Op: Operator<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		Pipe::new(self, operator)
	}
}

impl<T> ObservableExtensionPipe for T where T: Observable {}
