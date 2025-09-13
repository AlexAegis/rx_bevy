use rx_bevy_core::{Observable, Operator};

use crate::Pipe;

/// Provides a convenient function to pipe an operator from an observable
/// It's most useful for composite operators
pub trait ObservableExtensionPipe<'c>: Observable<'c> + Sized {
	fn pipe<Op>(self, operator: Op) -> Pipe<'c, Self, Op>
	where
		Self: Sized,
		Op: Operator<In = Self::Out, InError = Self::OutError>,
	{
		Pipe::new(self, operator)
	}
}

impl<'c, T> ObservableExtensionPipe<'c> for T where T: Observable<'c> {}
