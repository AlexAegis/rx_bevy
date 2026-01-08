use crate::{Observable, Operator};

pub trait ObservablePipeExtensionPipe: Observable + Sized + Send + Sync {
	fn pipe<'o, Op>(self, operator: Op) -> <Op as Operator<'o>>::OutObservable<Self>
	where
		Self: Sized,
		Op: Operator<'o, In = Self::Out, InError = Self::OutError>,
	{
		operator.operate(self)
	}
}

impl<O> ObservablePipeExtensionPipe for O where O: Observable + Send + Sync {}
