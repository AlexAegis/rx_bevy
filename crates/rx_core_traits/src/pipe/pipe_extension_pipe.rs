use crate::{Observable, Operator};

pub trait ObservablePipeExtensionPipe: Observable + Sized {
	fn pipe<Op>(self, operator: Op) -> Op::OutObservable<Self>
	where
		Self: Sized,
		Op: Operator<In = Self::Out, InError = Self::OutError>,
	{
		operator.operate(self)
	}
}

impl<O> ObservablePipeExtensionPipe for O where O: Observable {}
