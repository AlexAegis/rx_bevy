use rx_core_traits::{Observable, Observer, Operator};

use crate::operator::TapOperator;

pub trait ObservablePipeExtensionTap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn tap<TapDestination>(
		self,
		tap_destination: TapDestination,
	) -> <TapOperator<TapDestination> as Operator<'o>>::OutObservable<Self>
	where
		TapDestination:
			'static + Observer<In = Self::Out, InError = Self::OutError> + Clone + Send + Sync,
		Self::Out: Clone,
		Self::OutError: Clone,
	{
		TapOperator::new(tap_destination).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionTap<'o> for O where O: 'o + Observable + Send + Sync {}
