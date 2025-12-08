use rx_core_traits::{Observable, Operator};

use crate::observable::Pipe;

pub trait ObservablePipeExtensionPipe: Observable + Sized {
	fn pipe<Op>(self, operator: Op) -> Pipe<Self, Op>
	where
		Self: Sized,
		Op: Operator<In = Self::Out, InError = Self::OutError>,
	{
		Pipe::new(self, operator)
	}
}

impl<O> ObservablePipeExtensionPipe for O where O: Observable {}
