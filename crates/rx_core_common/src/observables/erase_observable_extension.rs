use crate::{ErasedObservable, Observable};

pub trait EraseObservableExtension<O>
where
	O: 'static + Observable + Send + Sync,
{
	fn erase(self) -> ErasedObservable<O::Out, O::OutError>;
}

impl<O> EraseObservableExtension<O> for O
where
	O: 'static + Observable + Send + Sync,
{
	#[inline]
	fn erase(self) -> ErasedObservable<<O>::Out, <O>::OutError> {
		ErasedObservable::new(self)
	}
}
