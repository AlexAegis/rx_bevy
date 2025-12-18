use rx_core_traits::{Observable, Operator};

use crate::operator::{ShareOperator, ShareOptions};

pub trait ObservablePipeExtensionShare: Observable + Sized {
	#[inline]
	fn share(
		self,
		options: ShareOptions<Self::Out, Self::OutError>,
	) -> <ShareOperator<Self::Out, Self::OutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Clone,
		Self::OutError: Clone,
	{
		ShareOperator::new(options).operate(self)
	}
}

impl<O> ObservablePipeExtensionShare for O where O: Observable {}
